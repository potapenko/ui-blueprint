package dev.uiblueprint.compose

import java.util.UUID

/** Explicit, UI-thread-owned session. IDs must be static public developer identifiers. */
class ProbeSession(
    val targetId: String,
    val surfaceId: String,
    val rootMarkerId: String,
    val platform: String,
    private val maxMarkers: Int = 256,
) : AutoCloseable {
    val sessionId: String = UUID.randomUUID().toString()
    val targetGeneration: String = UUID.randomUUID().toString()
    val surfaceGeneration: String = UUID.randomUUID().toString()
    private val thread = Thread.currentThread()
    private val epoch = System.nanoTime()
    private val markers = linkedMapOf<Any, ProbeMarker>()
    private var closed = false
    private var overflowMarkers = 0
    private var rootMount: String? = null
    private var sequence = 0L

    init {
        require(listOf(targetId, surfaceId, rootMarkerId).all(::publicId)) { "invalid_id" }
        require(platform == "desktop_macos" || platform == "android") { "unsupported_platform" }
        require(maxMarkers in 1..256) { "invalid_limit" }
    }

    internal fun checkThread() {
        check(Thread.currentThread() === thread) { "wrong_thread" }
        check(!closed) { "closed_session" }
    }

    internal fun register(owner: Any, marker: ProbeMarker): Boolean {
        checkThread()
        require(publicId(marker.id)) { "invalid_id" }
        if (markers.size >= maxMarkers && !markers.containsKey(owner)) {
            overflowMarkers++
            return false
        }
        markers[owner] = marker
        return true
    }

    internal fun unregister(owner: Any, admitted: Boolean = true) {
        check(Thread.currentThread() === thread) { "wrong_thread" }
        if (admitted) markers.remove(owner)
        else if (!closed) overflowMarkers--
    }

    /** No IO or polling. Invoke after layout from an explicitly authorized UI-thread callback. */
    fun capture(request: ProbeRequest): ByteArray {
        checkThread()
        check(overflowMarkers == 0) { "marker_limit" }
        require(publicId(request.requestId) && publicId(request.scopeId)) { "invalid_id" }
        require(request.maxNodes in 1..256 && request.maxOutputBytes in 1..1_048_576) { "invalid_limit" }
        require(request.deadlineMillis in 1..10_000) { "invalid_limit" }
        require(request.markerIds.isNotEmpty() && request.markerIds.size <= request.maxNodes) { "node_limit" }
        require(request.markerIds.all(::publicId) && request.markerIds.distinct().size == request.markerIds.size) { "invalid_selection" }
        require(rootMarkerId in request.markerIds) { "root_not_selected" }
        require(request.expectedMounts.keys.all { it in request.markerIds }) { "invalid_selection" }
        val start = System.nanoTime()
        fun deadline() { check(System.nanoTime() - start <= request.deadlineMillis * 1_000_000) { "deadline" } }
        fun selected(id: String): ProbeMarker {
            val matches = markers.values.filter { it.id == id }
            check(matches.size == 1) { if (matches.isEmpty()) "target_unresolved" else "ambiguous_target" }
            return matches.single().also {
                check(it.attached()) { "stale_target" }
                check(request.expectedMounts[id]?.let { expected -> expected == it.mountId } != false) { "stale_target" }
            }
        }
        val root = selected(rootMarkerId)
        check(rootMount == null || rootMount == root.mountId) { "stale_surface" }
        rootMount = root.mountId
        val density = root.density()
        require(density.first.isFinite() && density.first > 0 && density.second.isFinite() && density.second > 0) { "invalid_density" }
        val samples = request.markerIds.map { id ->
            deadline()
            val marker = selected(id)
            val sample = marker.read(root)
            check(marker.attached()) { "stale_target" }
            ProbeSample(id, marker.mountId, sample)
        }
        deadline()
        val end = System.nanoTime()
        sequence = Math.addExact(sequence, 1)
        return encodeProbe(this, request, samples, density, sequence,
            (start - epoch) / 1_000_000.0, (end - epoch) / 1_000_000.0).also { deadline() }
    }

    override fun close() {
        check(Thread.currentThread() === thread) { "wrong_thread" }
        markers.clear()
        overflowMarkers = 0
        closed = true
    }
}

data class ProbeRequest(
    val requestId: String,
    val scopeId: String,
    val markerIds: List<String>,
    val maxNodes: Int,
    val maxOutputBytes: Int,
    val deadlineMillis: Long,
    val expectedMounts: Map<String, String> = emptyMap(),
)

internal fun publicId(value: String): Boolean = value.length in 1..96 &&
    value.all { it in 'a'..'z' || it in 'A'..'Z' || it in '0'..'9' || it in "._-" }

internal interface ProbeMarker {
    val id: String
    val mountId: String
    fun attached(): Boolean
    fun density(): Pair<Float, Float>
    fun read(root: ProbeMarker): ProbeBounds?
}
internal data class ProbeBounds(val x: Float, val y: Float, val width: Float, val height: Float,
    val density: Float, val fontScale: Float)
internal data class ProbeSample(val id: String, val mountId: String, val bounds: ProbeBounds?)
