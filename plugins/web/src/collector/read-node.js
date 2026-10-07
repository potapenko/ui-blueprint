function readNode(options, expectedDocument) {
  'use strict';
  // The caller resolves exactly one backend node in this isolated world.
  // Arguments are data; there is no selector, eval, page callback, await or mutation.
  const fields = new Set(options.fields);
  const owns = this instanceof Element && this.ownerDocument === expectedDocument && expectedDocument === document;
  const connected = owns && this.isConnected;
  const empty = { connected, sameDocument: owns, tag: null, sensitive: true };
  if (!connected) return empty;
  const text = s => typeof s === 'string' && s.length <= options.maxChars ? s : null;
  const attr = name => Element.prototype.getAttribute.call(this, name);
  const input = this instanceof HTMLInputElement;
  const area = this instanceof HTMLTextAreaElement;
  const select = this instanceof HTMLSelectElement;
  const option = this instanceof HTMLOptionElement;
  const button = this instanceof HTMLButtonElement;
  const native = (prototype, name) => Object.getOwnPropertyDescriptor(prototype, name).get.call(this);
  const type = input ? native(HTMLInputElement.prototype, 'type') : null;
  const autocomplete = attr('autocomplete');
  const privateAutocomplete = autocomplete !== null && (autocomplete.length > 128 ||
    /(?:^|\s)(?:current-password|new-password|one-time-code|cc-number|cc-csc)(?:\s|$)/i.test(autocomplete));
  const sensitive = options.sensitive || type === 'password' || privateAutocomplete;
  const out = { connected, sameDocument: owns, tag: sensitive ? null : text(this.tagName), sensitive };
  if (!sensitive) {
    // Resolve IDREFs only to the original, explicitly selected objects. Browser
    // ID lookup confirms the endpoint; a matching string alone is not identity.
    const selected = Array.prototype.slice.call(arguments, 2);
    const endpoint = id => {
      if (!id || id.length > options.maxChars || /[\t\n\f\r ]/.test(id)) return null;
      let index = null;
      for (let i = 0; i < selected.length; i++) {
        const node = selected[i];
        if (!(node instanceof Element) || !node.isConnected || node.ownerDocument !== expectedDocument) continue;
        if (Element.prototype.getAttribute.call(node, 'id') === id) {
          if (index !== null) return null;
          index = i;
        }
      }
      return index !== null && Document.prototype.getElementById.call(expectedDocument, id) === selected[index] ? index : null;
    };
    const controls = text(attr('aria-controls'));
    if (controls !== null) {
      const ids = controls.trim().split(/[\t\n\f\r ]+/);
      if (ids.length <= selected.length) out.controls = [...new Set(ids.map(endpoint).filter(index => index !== null))];
    }
    // data-anchor is the explicit fixture declaration, not an ARIA synonym.
    out.declaredAnchor = endpoint(text(attr('data-anchor')));
    if (fields.has('focused') && document.activeElement === this)
      out.activeDescendant = endpoint(text(attr('aria-activedescendant')));
  }
  if (fields.has('layout_bounds')) {
    // CSSOM's native layout/fragments computation is opaque browser work.
    // Empty fragment list is unavailable layout, never a fabricated zero rectangle.
    const fragments = Element.prototype.getClientRects.call(this);
    if (fragments.length > 0) {
      const r = Element.prototype.getBoundingClientRect.call(this);
      out.rect = { x: r.x, y: r.y, width: r.width, height: r.height };
    }
  }
  if (!sensitive) {
    if (fields.has('input_kind')) out.inputKind = input ? type : area ? 'textarea' : select ? 'select' : null;
    if (fields.has('value') && (input || area || select)) {
      const prototype = input ? HTMLInputElement.prototype : area ? HTMLTextAreaElement.prototype : HTMLSelectElement.prototype;
      out.value = text(native(prototype, 'value'));
    }
    if (fields.has('placeholder') && (input || area)) {
      out.placeholder = text(native(input ? HTMLInputElement.prototype : HTMLTextAreaElement.prototype, 'placeholder'));
    }
  }
  if (fields.has('required') && (input || area || select)) out.required = native(input ? HTMLInputElement.prototype : area ? HTMLTextAreaElement.prototype : HTMLSelectElement.prototype, 'required');
  if (fields.has('readonly') && (input || area)) out.readonly = native(input ? HTMLInputElement.prototype : HTMLTextAreaElement.prototype, 'readOnly');
  if (fields.has('enabled') && (input || area || select || button || option)) out.enabled = !Element.prototype.matches.call(this, ':disabled');
  if (fields.has('checked') && input && (type === 'checkbox' || type === 'radio')) {
    if (!native(HTMLInputElement.prototype, 'indeterminate')) out.checked = native(HTMLInputElement.prototype, 'checked');
  }
  if (fields.has('selected') && option) out.selected = native(HTMLOptionElement.prototype, 'selected');
  if (fields.has('focused')) out.focused = document.activeElement === this;
  if (fields.has('expanded')) {
    const v = attr('aria-expanded');
    if (v === 'true' || v === 'false') out.expanded = v === 'true';
  }
  if (fields.has('invalid') && (input || area || select)) {
    const validity = native(input ? HTMLInputElement.prototype : area ? HTMLTextAreaElement.prototype : HTMLSelectElement.prototype, 'validity');
    out.invalid = !validity.valid;
  }
  return out;
}
