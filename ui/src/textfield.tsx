import { h } from 'hyperapp';

export const Textfield = ({ text, oninput }) => (
  <div
    class="ms-TextField ms-TextField--multiline"
    oncreate={el => new fabric.TextField(el)}
    style={{ background: '#cccccc', height: '600px', borderRadius: '10px' }}
  >
    <label class="ms-Label">Editor</label>
    <textarea class="ms-TextField-field" value={text} oninput={oninput} style={{ background: 'black', height: '100%', color: 'white' }} />
  </div>
);
