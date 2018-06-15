import { h } from 'hyperapp';

export const Panel = () => (
  <div onClick class="ms-PanelExample" oncreate={_ => (Panel['p'] = document.querySelector('.ms-Panel'))} style={{ display: 'flex' }}>
    <button class="ms-Button ms-Button--hero" onclick={_ => new fabric.Panel(Panel['p'])} style={{ minWidth: 0, margin: "10px" }}>
      <span class="ms-Button-icon">
        <i class="ms-Icon ms-Icon--Add" />
      </span>
    </button>
  </div>
);
