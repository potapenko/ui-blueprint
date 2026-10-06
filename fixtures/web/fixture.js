// Synthetic application/controller. No collector code or expected.json dependency.
(() => {
  const $ = selector => document.querySelector(selector);
  const generation = Number(new URL(location.href).searchParams.get('generation') || '1');
  let revision = 0, seq = 0, sourceState = 'initial', timer;
  let selected = '', applied = '', valid = false, delivered = 0;
  const events = [];
  function checkpoint() {
    return { generation, revision, source_state: sourceState, selected, applied, valid, delivered,
      draft: $('#draft').value, rows: [...document.querySelectorAll('[data-row]')].map(n => ({ id: n.dataset.row, label: n.textContent })) };
  }
  function bump(name) {
    sourceState = name; revision++;
    document.documentElement.dataset.sourceState = name;
    const event = { seq: ++seq, generation, revision, source_state: name };
    events.push(event); document.dispatchEvent(new CustomEvent('fixture-change', { detail: event }));
  }
  $('#left').dataset.componentKey = 'apply-control';
  $('#left').children[0].id = 'compound-icon';
  $('#left').children[1].id = 'compound-label';
  $('#draft').addEventListener('input', () => {
    clearTimeout(timer); selected = ''; valid = false;
    $('#suggestions').replaceChildren(); $('#validation').textContent = 'Pending';
    $('#draft').setAttribute('aria-expanded', 'false'); bump('draft-pending');
    timer = setTimeout(() => {
      const query = $('#draft').value;
      if (query === 'Lon' || query === 'London') {
        const option = document.createElement('button');
        option.id = 'option-london'; option.setAttribute('role', 'option'); option.textContent = 'London';
        option.onclick = () => {
          selected = 'London'; $('#draft').value = selected; valid = true;
          $('#draft').setAttribute('aria-expanded', 'false');
          $('#draft').setAttribute('aria-invalid', 'false');
          $('#suggestions').replaceChildren(); $('#validation').textContent = 'Valid'; bump('city-selected');
        };
        $('#suggestions').append(option); $('#draft').setAttribute('aria-expanded', 'true');
        $('#validation').textContent = 'Choose a city'; bump('suggestion-ready');
      } else {
        $('#draft').setAttribute('aria-invalid', 'true'); $('#validation').textContent = 'Invalid city'; bump('draft-invalid');
      }
    }, 120);
  });
  $('#commit').onclick = () => {
    delivered++;
    if (!valid) { $('#validation').textContent = 'Select a suggestion first'; bump('commit-rejected'); return; }
    applied = selected; $('#applied').textContent = applied; bump('city-applied');
  };
  $('#open-popup').onclick = () => {
    if ($('#portal')) return;
    const portal = document.createElement('div'); portal.id = 'portal'; portal.setAttribute('role', 'dialog');
    portal.setAttribute('aria-label', 'Options'); portal.dataset.anchor = 'open-popup';
    const close = document.createElement('button'); close.id = 'close-popup'; close.textContent = 'Close options';
    close.onclick = () => { portal.remove(); $('#open-popup').setAttribute('aria-expanded', 'false'); $('#open-popup').focus(); bump('popup-closed'); };
    portal.append(close); document.body.append(portal); $('#open-popup').setAttribute('aria-expanded', 'true'); close.focus(); bump('popup-open');
  };
  $('#surprise').onclick = () => {
    const dialog = document.createElement('div'); dialog.id = 'unexpected'; dialog.setAttribute('role', 'dialog');
    dialog.setAttribute('aria-label', 'Unexpected state'); dialog.textContent = 'Refresh observation';
    document.body.append(dialog); $('#commit').disabled = true; bump('unexpected-transition');
  };
  const operations = {
    remount() { const left = $('#left'); left.replaceWith(left.cloneNode(true)); bump('left-remounted'); },
    overlay() { const cover = document.createElement('div'); cover.id = 'cover'; document.body.append(cover); bump('overlay-on'); },
    removeOverlay() { $('#cover')?.remove(); bump('overlay-off'); },
    textLarge() { $('#sized').style.fontSize = '24px'; bump('text-large'); },
    locale() { document.documentElement.lang = 'fr'; $('#responsive').textContent = 'Adaptatif'; bump('locale-fr'); },
    parentWide() { $('#mutation-parent').style.width = '300px'; bump('parent-wide'); },
    fontLarge() { $('#mutation-parent').style.fontSize = '24px'; bump('font-large'); },
    rowsChanged() {
      $('[data-row="r2"]').textContent = 'Deux'; $('[data-row="r3"]').remove();
      const row = document.createElement('div'); row.dataset.row = 'r7'; row.textContent = 'Seven'; $('#rows').append(row); bump('rows-changed');
    }
  };
  document.documentElement.dataset.sourceState = sourceState;
  window.f01 = { checkpoint, events, operate(name) {
    if (!Object.hasOwn(operations, name)) throw new Error('Unknown fixture stimulus'); operations[name](); return checkpoint();
  } };
})();
