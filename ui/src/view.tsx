import { h, View } from 'hyperapp';
import State from './state';
import { Actions } from './actions';
import { Panel } from './panel';
import { Textfield } from './textfield';

export const view: View<State, Actions> = (state, actions) => (
  <main>
    <Panel />
    <Textfield text={state.text} oninput={e => actions.setText(e.target.value)} />
    <br />
    <div
      style={{
        display: 'flex',
        backgroundColor: '#cccccc',
        borderRadius: '5px',
        padding: '20px 30px'
      }}
    >
      <button class="ms-Button" onclick={_ => actions.openDialog()}>
        <span class="ms-Button-label">open</span>
      </button>
      <button class="ms-Button" onclick={_ => actions.getFile('c://users/dellepiane/desktop/hyper.js')}>
        <span class="ms-Button-label">load</span>
      </button>
      <button class="ms-Button" onclick={_ => actions.setText('')}>
        <span class="ms-Button-label">reset</span>
      </button>
    </div>
  </main>
);
