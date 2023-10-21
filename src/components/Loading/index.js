import { render, unmountComponentAtNode } from 'react-dom';
import './index.css';

const DOM_ID = 'loading';

export default function Loading() {
  return (
    <div className="loading-container">
      <div className="lds-roller"><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div></div>
    </div>
  );
}

Loading.show = () => {
  const loadingRoot = document.getElementById(DOM_ID);
  render(<Loading />, loadingRoot);
}

Loading.hide = () => {
  const loadingRoot = document.getElementById(DOM_ID);
  unmountComponentAtNode(loadingRoot);
}

window.loading = Loading;