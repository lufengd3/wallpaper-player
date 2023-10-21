import React from 'react';
import ReactDOM from 'react-dom';
import { RecoilRoot } from 'recoil';

import './webcomponents';
import App from './App';
import TitleBar from './components/TitleBar';
import './index.css';

ReactDOM.render(
  <React.StrictMode>
    <RecoilRoot>
      <TitleBar />
      <App />
    </RecoilRoot>
  </React.StrictMode>,
  document.getElementById('root')
);

document.addEventListener('contextmenu', event => event.preventDefault());