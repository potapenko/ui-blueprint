function verifyNodes(expectedDocument) {
  'use strict';
  // The caller supplies the original remote objects, never a locator re-query.
  // This bounded identity check does not reread values, names, layout or state.
  if (expectedDocument !== document) return { current: false };
  for (let index = 1; index < arguments.length; index++) {
    const node = arguments[index];
    if (!(node instanceof Element) || !node.isConnected || node.ownerDocument !== expectedDocument) {
      return { current: false };
    }
  }
  return { current: true };
}
