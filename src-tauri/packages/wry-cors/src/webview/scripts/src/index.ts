import BackBtn from './BackBtn';
import ContextMenu from './ContextMenu';
import processTagA from './utils/tag-a-processor';

function _wp_page_init() {
  new BackBtn();
  new ContextMenu();
  processTagA();
}

try {
  if (!/localhost/.test(location.host)) {
    window.addEventListener('DOMContentLoaded', () => {
      _wp_page_init();
    });
  }
} catch(e) {
  console.log('wp code error', e);
}