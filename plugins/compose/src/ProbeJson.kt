package dev.uiblueprint.compose

/** Fixed, text-free DTO encoder. All strings admitted by publicId or generated UUIDs. */
internal fun encodeProbe(session: ProbeSession, request: ProbeRequest, samples: List<ProbeSample>,
    density: Pair<Float, Float>, sequence: Long, start: Double, end: Double): ByteArray {
    val out = StringBuilder()
    fun append(value: String) {
        check(out.length + value.length <= request.maxOutputBytes) { "output_limit" }
        out.append(value)
    }
    append("{\"probe_version\":\"1.0.0\",\"session_id\":\"${session.sessionId}\",\"target\":{\"id\":\"${session.targetId}\",\"generation\":\"${session.targetGeneration}\"},")
    append("\"surface\":{\"id\":\"${session.surfaceId}\",\"generation\":\"${session.surfaceGeneration}\"},\"platform\":\"${session.platform}\",")
    append("\"request_id\":\"${request.requestId}\",\"scope_id\":\"${request.scopeId}\",\"root_marker_id\":\"${session.rootMarkerId}\",\"sequence\":$sequence,")
    append("\"start_ms\":$start,\"end_ms\":$end,\"density\":${density.first.toDouble()},\"font_scale\":${density.second.toDouble()},\"nodes\":[")
    samples.forEachIndexed { index, sample ->
        if (index != 0) append(",")
        append("{\"marker_id\":\"${sample.id}\",\"mount_id\":\"${sample.mountId}\",\"bounds\":")
        val b = sample.bounds
        if (b == null) append("null") else append("{\"x\":${b.x.toDouble()},\"y\":${b.y.toDouble()},\"width\":${b.width.toDouble()},\"height\":${b.height.toDouble()},\"density\":${b.density.toDouble()},\"font_scale\":${b.fontScale.toDouble()}}")
        append("}")
    }
    append("]}\n")
    return out.toString().toByteArray(Charsets.UTF_8)
}
