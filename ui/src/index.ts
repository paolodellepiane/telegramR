import 'core-js';
import 'whatwg-fetch';
import '../styles/main.scss';
import { app } from 'hyperapp';
import { fabric } from 'office-ui-fabric-js/dist/js/fabric.min';
import { view } from './view';
import { actions } from './actions';
import State from './state';
import { withLogger } from '@hyperapp/logger';
import './rpc';

window['fabric'] = fabric;

export const initialState: State = {
  filePath: '',
  text: 'empty'
};

withLogger(app)(initialState, actions, view, document.body);

// let main;
// if (!config.devtools || config.devtools === 'none') main = withLogger(app)(initialState, actions, view, document.body);
// else
//   switch (config.devtools) {
//     case 'hyperapp-redux-devtools':
//       import('hyperapp-redux-devtools').then(
//         devtools => (main = withLogger(devtools.default(app))(initialState, actions, view, document.body))
//       );
//       break;
//     case 'hyperapp-devtools':
//       import('hyperapp-devtools').then(
//         devtools => (main = withLogger(devtools.default(app))(initialState, actions, view, document.body))
//       );
//       break;
//   }
