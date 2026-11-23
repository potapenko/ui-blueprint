"""A partial geometry projection, after the caller proves source/native identity.

No memory reads, geometry arithmetic, source mutation or alternate acquisition.
The caller retains structural traversal and native-witness union verification.
"""
from swiftui_debug_snapshot import Refusal


class GeometryFrontier:
    def __init__(self, depth, source_depth_limit):
        if (type(depth) is not int or type(source_depth_limit) is not int
                or not 1 <= depth <= source_depth_limit <= 256):
            raise Refusal('invalid_geometry_frontier', 2)
        self.depth = depth
        self.source_depth_limit = source_depth_limit
        self.seen = set()
        self.boundaries = []

    def advance(self, element, parent_depth, fields, admit):
        """Return (branch geometry depth, cut); never inspect cached values.

        admit is the existing raw-budget gate, including ancestor/context reuse.
        Every view record is visited once; transparent-record cycles stay the
        surrounding traversal's responsibility.
        """
        if (type(element) is not int or element <= 0 or type(parent_depth) is not int
                or not 0 <= parent_depth <= self.source_depth_limit):
            raise Refusal('invalid_geometry_frontier', 2)
        if element in self.seen:
            raise Refusal('geometry_frontier_cycle')
        admit(element)
        bearing = any(field[0] in ('position', 'size') for field in fields)
        depth = parent_depth + int(bearing)
        if depth > self.source_depth_limit:
            raise Refusal('geometry_frontier_depth', 2)
        self.seen.add(element)
        return depth, bearing and depth >= self.depth

    def boundary(self, node_index, depth, witness_only):
        if (type(node_index) is not int or not 0 <= node_index < 256
                or type(depth) is not int or not self.depth <= depth <= self.source_depth_limit
                or type(witness_only) is not bool):
            raise Refusal('invalid_geometry_frontier_boundary', 2)
        self.boundaries.append(dict(node_index=node_index, geometry_depth=depth,
                                    native_witness_context=witness_only))

    def manifest(self):
        return dict(depth=self.depth, boundaries=[dict(item) for item in self.boundaries],
                    coverage='partial', deeper_children='excluded_except_native_witness_paths')
