package dev.uiblueprint.compose

import androidx.compose.ui.layout.LayoutCoordinates
import java.lang.reflect.Proxy

private fun coordinates(parent: LayoutCoordinates?, layoutParent: LayoutCoordinates?) =
    Proxy.newProxyInstance(LayoutCoordinates::class.java.classLoader, arrayOf(LayoutCoordinates::class.java)) { _, method, _ ->
        when (method.name) {
            "getParentCoordinates" -> parent
            "getParentLayoutCoordinates" -> layoutParent
            else -> error("unexpected_coordinate_read")
        }
    } as LayoutCoordinates


private class Marker(override val id: String, override val mountId: String) : ProbeMarker {
    var live = true
    var value: ProbeBounds? = ProbeBounds(0f, 0f, 200f, 100f, 2f, 1f)
    var reads = 0
    override fun attached() = live
    override fun density() = 2f to 1f
    override fun read(root: ProbeMarker): ProbeBounds? { reads++; return value }
}
fun main() {
    val outer = coordinates(null, null)
    val markedRoot = coordinates(outer, outer)
    val childCoordinates = coordinates(markedRoot, outer)
    check(withinRoot(childCoordinates, markedRoot)) // old parentLayoutCoordinates chain skipped root
    check(withinRoot(markedRoot, markedRoot))
    check(!withinRoot(childCoordinates, coordinates(null, null)))
    var deep = childCoordinates
    repeat(129) { deep = coordinates(deep, outer) }
    check(!withinRoot(deep, markedRoot))

    val session = ProbeSession("playphraseme-test", "main", "root", "desktop_macos", 4)
    val root = Marker("root", "root-mount")
    val child = Marker("video", "video-mount")
    session.register(root, root); session.register(child, child)
    val request = ProbeRequest("first", "player", listOf("root", "video"), 2, 16384, 1000)
    check(root.reads == 0 && child.reads == 0)
    val overflowSession = ProbeSession("test", "main", "root", "android", 1)
    check(overflowSession.register(root, root))
    check(!overflowSession.register(child, child)) // registration never disrupts UI on cap
    check(runCatching { overflowSession.capture(request) }.exceptionOrNull()?.message == "marker_limit")
    overflowSession.unregister(child, admitted = false)
    overflowSession.close()
    val bytes = session.capture(request)
    check(root.reads == 1 && child.reads == 1)
    check(String(bytes).contains("\"width\":200.0"))
    fun refused(code: String, block: () -> Unit) {
        val result = runCatching(block).exceptionOrNull()
        check(result is IllegalStateException || result is IllegalArgumentException)
        check(result.message == code) { "expected $code, got ${result.message}" }
    }
    refused("output_limit") { session.capture(request.copy(maxOutputBytes = 16)) }
    refused("node_limit") { session.capture(request.copy(maxNodes = 1)) }
    refused("stale_target") { session.capture(request.copy(expectedMounts = mapOf("video" to "old"))) }
    val duplicate = Marker("video", "duplicate")
    session.register(duplicate, duplicate)
    refused("ambiguous_target") { session.capture(request) }
    session.unregister(duplicate)
    child.live = false
    refused("stale_target") { session.capture(request) }
    child.live = true
    session.unregister(child)
    refused("target_unresolved") { session.capture(request) }
    session.register(child, child)
    child.value = null
    check(String(session.capture(request)).contains("\"bounds\":null"))
    session.unregister(root)
    val newRoot = Marker("root", "new-root")
    session.register(newRoot, newRoot)
    refused("stale_surface") { session.capture(request) }
    session.close()
    refused("closed_session") { session.capture(request) }
    println("Compose session lifecycle/limits: passed")
    // Collector-generated DTO for the separate Rust integration test; synthetic by design.
    print(String(bytes))
}
