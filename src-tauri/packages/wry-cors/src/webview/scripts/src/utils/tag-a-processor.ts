import { PREVIEW_IFRAME_ID, PREVIEW_IFRAME_ZINDEX } from '../constants';

export default () => {
  var processAnchor = function(a) {
    a.removeAttribute('target');

    if (/cdn.wallpaperhub.app/i.test(a.search)) {
        const params = new URLSearchParams(a.search);
        const imgUrl = params.get('url');
        if (imgUrl) {
            a.removeAttribute('href');
            a.onmouseover = () => {
                a.style.cursor = 'pointer';
            }
            a.onmouseout = () => {
                a.style.cursor = '';
            }
            a.onclick = () => {
                const imgIframe = document.createElement('ifarme');
                const htmlContent = `
                    <!DOCTYPE html>
                        <body>
                            <img src="${imgUrl}" style="width: 100%;" />
                        </body>
                    </html>
                `;
                imgIframe.innerHTML = htmlContent;
                imgIframe.setAttribute('id', PREVIEW_IFRAME_ID);
                imgIframe.style.width = '100vw';
                imgIframe.style.height = '100vh';
                imgIframe.style.position = 'fixed';
                imgIframe.style.zIndex = PREVIEW_IFRAME_ZINDEX;
                imgIframe.style.top = 0;
                imgIframe.style.left = 0;
                document.body.appendChild(imgIframe);
            }
        }
    }
  };
  var insertedObserver = new MutationObserver(function(mutations) {
      mutations.forEach(function(m) {
          var inserted = [].slice.call(m.addedNodes);
          while (inserted.length > 0) {
              var elem = inserted.shift();
              [].slice.call(elem.children || []).forEach(function(el) {
                  inserted.push(el);
              });
              if (elem.nodeName === 'A') {
                  processAnchor(elem);
              }
          }
      });
  });
  insertedObserver.observe(document.documentElement, {
      childList: true,
      subtree: true
  });

  var blankLinks = document.querySelectorAll('a[target=_blank]');
  if (blankLinks) {
    blankLinks.forEach(processAnchor);
  }
}