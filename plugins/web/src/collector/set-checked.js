function setChecked(expectedDocument, value) {
  'use strict';
  if (!(this instanceof Element) || !this.isConnected || this.ownerDocument !== expectedDocument || expectedDocument !== document)
    return {status: 'stale'};
  if (!(this instanceof HTMLInputElement) || typeof value !== 'boolean') return {status: 'unsupported'};
  const autocomplete = Element.prototype.getAttribute.call(this, 'autocomplete');
  if (autocomplete !== null && (autocomplete.length > 128 || /(?:^|\s)(?:current-password|new-password|one-time-code|cc-number|cc-csc)(?:\s|$)/i.test(autocomplete))) return {status: 'unsupported'};
  const native = name => Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, name).get.call(this);
  const checked = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'checked');
  if (native('type') !== 'checkbox' || typeof checked.set !== 'function') return {status: 'unsupported'};
  if (Element.prototype.matches.call(this, ':disabled')) return {status: 'disabled'};
  if (native('indeterminate')) return {status: 'indeterminate'};
  // One native Setter invocation, even when already equal. No toggle, events or input.
  checked.set.call(this, value);
  return {status: 'applied'};
}
