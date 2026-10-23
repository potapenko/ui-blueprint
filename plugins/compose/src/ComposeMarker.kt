package dev.uiblueprint.compose

import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.LayoutCoordinates
import androidx.compose.ui.node.GlobalPositionAwareModifierNode
import androidx.compose.ui.node.ModifierNodeElement
import androidx.compose.ui.node.requireDensity
import androidx.compose.ui.platform.InspectorInfo
import java.util.UUID

/** Registers only this existing layout; adds no layout/draw/semantics/input modifier. */
fun Modifier.uiBlueprintMarker(session: ProbeSession, id: String): Modifier =
    this.then(MarkerElement(session, id))

private data class MarkerElement(val session: ProbeSession, val id: String) : ModifierNodeElement<MarkerNode>() {
    init { require(publicId(id)) { "invalid_id" } }
    override fun create() = MarkerNode(session, id)
    override fun update(node: MarkerNode) = node.update(session, id)
    override fun InspectorInfo.inspectableProperties() { name = "uiBlueprintMarker" }
}

private class MarkerNode(var session: ProbeSession, override var id: String) :
    Modifier.Node(), GlobalPositionAwareModifierNode, ProbeMarker {
    override var mountId = UUID.randomUUID().toString()
    private var coordinates: LayoutCoordinates? = null
    private var admitted = false

    override fun onAttach() {
        mountId = UUID.randomUUID().toString()
        admitted = session.register(this, this)
    }
    override fun onDetach() {
        session.unregister(this, admitted)
        coordinates = null
    }
    override fun onReset() {
        mountId = UUID.randomUUID().toString()
        coordinates = null
    }
    fun update(nextSession: ProbeSession, nextId: String) {
        if (nextSession === session && nextId == id) return
        if (isAttached) session.unregister(this, admitted)
        session = nextSession
        id = nextId
        mountId = UUID.randomUUID().toString()
        if (isAttached) admitted = session.register(this, this)
    }
    override fun onGloballyPositioned(coordinates: LayoutCoordinates) { this.coordinates = coordinates }
    override fun attached() = isAttached && coordinates?.isAttached == true
    override fun density() = requireDensity().let { it.density to it.fontScale }

    override fun read(root: ProbeMarker): ProbeBounds? {
        val current = coordinates ?: return null
        val rootCoordinates = (root as? MarkerNode)?.coordinates ?: return null
        // A selected root must be an actual ancestor, not another window/hierarchy.
        check(withinRoot(current, rootCoordinates)) { "outside_root" }
        return try {
            val bounds = rootCoordinates.localBoundingBoxOf(current, clipBounds = false)
            val density = density()
            val values = listOf(bounds.left, bounds.top, bounds.width, bounds.height, density.first, density.second)
            if (values.any { !it.isFinite() } || bounds.width < 0 || bounds.height < 0 || density.first <= 0 || density.second <= 0) null
            else ProbeBounds(bounds.left, bounds.top, bounds.width, bounds.height, density.first, density.second)
        } catch (_: IllegalStateException) { null }
    }
}

/** Includes modifier coordinators; parentLayoutCoordinates would skip marked root modifiers. */
internal fun withinRoot(current: LayoutCoordinates, root: LayoutCoordinates): Boolean {
    var ancestor: LayoutCoordinates? = current
    var depth = 0
    while (ancestor !== root && ancestor != null && depth++ < 128) {
        ancestor = ancestor.parentCoordinates
    }
    return ancestor === root
}
