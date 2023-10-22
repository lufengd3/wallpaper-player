const ADD_TO_QUEUE = '93984302';
const SET_WP = '12798371231';
const COPY = '123892x003';
const MENU_WIDTH = '120px';
const IMG_INFO_DOM_ID = '_wpContextMenuImgSize';
const CTX_MENU_BODY_ID = '_wpContextMenuBody';

export default class ContextMenu {
  dom: HTMLElement;
  currentUrl: string;
  base64Img: string;

  constructor() {
    this.dom = this.renderDOM();
    this.appendCss();
    this.addEventListener();
  }

  renderDOM = (): HTMLElement => {
    var contextMenu = document.createElement('div');
    contextMenu.setAttribute('class', '_wpContextMenuContainer');
    contextMenu.addEventListener('click', this.handleMenuClick);
   
    contextMenu.innerHTML = `
    <ul id="${CTX_MENU_BODY_ID}">
      <li data-action="${SET_WP}">设为壁纸</li>
      <li data-action="${ADD_TO_QUEUE}">加入下载队列</li>
      <li data-action="${COPY}">复制图片 URL</li>
    </ul>
    `;

    document.body.appendChild(contextMenu);
    return contextMenu;
  }

  appendCss = (): void => {
    var contextMenuCss = `
      ._wpContextMenuContainer {
        width: 100vw;
        height: 100vh;
        position: fixed;
        top: 0;
        left: 0;
        z-index: 9999999;
        display: none;
      }
      #${CTX_MENU_BODY_ID} {
        padding: 0 10px;
        background-color: #f4f4f4;
        color: #333;
        box-shadow: 1px 1px 5px 1px rgba(0,0,0,0.2);
        width: ${MENU_WIDTH};
        display: inline-flex;
        flex-direction: column;
        position: absolute;
      }

      #${IMG_INFO_DOM_ID} {
        color: #999;
      }

      #${IMG_INFO_DOM_ID}:hover {
        cursor: context-menu;
        font-weight: normal;
      }

      #${CTX_MENU_BODY_ID} li {
        height: 40px;
        line-height: 40px;
        text-align: left;
        list-style: none;
      }
      
      #${CTX_MENU_BODY_ID} li:hover {
        cursor: pointer;
        font-weight: bold;
      }
      
      #${CTX_MENU_BODY_ID} li:not(:last-child) {
        border-bottom: 0.5px solid rgba(200, 200, 200, 0.5);
      }

      #${CTX_MENU_BODY_ID} li:hover {
        cursor: pointer;
      }
    `;
    var wpHhostStyle = document.createElement('style');
    wpHhostStyle.innerHTML = contextMenuCss;
    document.head.appendChild(wpHhostStyle);
  }

  addEventListener = (): void => {
    window.addEventListener('contextmenu', (e: PointerEvent) => {
      const {tagName, src, style} = e.target as HTMLImageElement;
      // console.log(tagName)
      // wallhaven
      // https://wallhaven.cc/w/nk5576
      // https://w.wallhaven.cc/full/nk/wallhaven-nk5576.jpg
      // if () {

      //   return;
      // }
      if (/img/i.test(tagName) && src) {
        const realUrl = this.handleImgUrl(src, e);
        if (realUrl) {
          e.preventDefault();
          this.generateImgBase64Data(e.target as HTMLImageElement);
        }
      } else if (/http/i.test(style.backgroundImage)) {
        const url = style.backgroundImage.replace(/["']/g, "").slice(4, -1);
        const realUrl = this.handleImgUrl(url, e);
        if (realUrl) {
          e.preventDefault();
          const img = new Image();
          img.src = realUrl;
          img.onload = () => {
            this.generateImgBase64Data(img);
          }
        }
      }
    });

    document.getElementById(CTX_MENU_BODY_ID).addEventListener('click', (e: PointerEvent) => {
      const { dataset } = e.target as HTMLImageElement;
      switch (dataset.action) {
        case SET_WP:
          this.setWallpaper();
          break;
        case ADD_TO_QUEUE:
          this.downloadWallpaper();
          break;
        case COPY:
          this.copyUrl();
          break;
        default:
          break;
      }
      this.hideMenu();
    });
  }

  generateImgBase64Data = (img: HTMLImageElement) => {
    if (img.naturalWidth >= 1920) {
      const canvas = document.createElement("canvas");
      canvas.width = img.naturalWidth;
      canvas.height = img.naturalHeight;
      const ctx = canvas.getContext("2d");
      ctx.drawImage(img, 0, 0);
      this.base64Img = canvas.toDataURL("image/jpeg", 0.99);
    } else {
      this.base64Img = '';
    }

  }

  handleImgUrl = (url: String, e: PointerEvent) => {
    if (!url) {
      this.currentUrl = '';
      return;
    };
    
    this.currentUrl = url.replace(/&(h|height)=\d+/g, '').replace(/&(w|width)=\d+/g, '');
    this.showMenu(e.clientX, e.clientY);
    
    const menuBody =  document.getElementById(CTX_MENU_BODY_ID);
    if (menuBody && !/unsplash\.com/.test(location.host)) {
      const img = e.target as HTMLImageElement;
      const {naturalWidth = 0, naturalHeight = 0} = img;
      const imgInfoElm = document.createElement('li');
      imgInfoElm.setAttribute('id', IMG_INFO_DOM_ID);
      imgInfoElm.textContent = naturalWidth ? `${naturalWidth} x ${naturalHeight}` : 'Unknow';
      menuBody.insertBefore(imgInfoElm, menuBody.firstChild);
    }

    return this.currentUrl;
  }

  handleMenuClick = (e: Event) => {
    const elm = e.target as HTMLElement;
    console.log(e.target);
    if (elm === this.dom) {
      this.hideMenu();
      return;
    }

    console.log(elm.dataset.action);
  }

  showMenu = (x: number, y: number) => {
    this.dom.style.display = 'block';

    const menuBody = document.getElementById(CTX_MENU_BODY_ID);
    const menuWidth = parseInt(MENU_WIDTH);
    const menuHeight = 125;
    const docWidth = document.documentElement.offsetWidth;
    const docHeight = document.documentElement.offsetHeight;
    const left = x + menuWidth > docWidth ? x - menuWidth : x;
    const top = y + menuHeight > docHeight ? y - menuHeight : y;
    menuBody.style.left = left + 'px';
    menuBody.style.top = top + 'px';
  }

  hideMenu = () => {
    const imgInfoElm = document.getElementById(IMG_INFO_DOM_ID);
    imgInfoElm && imgInfoElm.remove();
    this.dom.style.display = 'none';
  }

  setWallpaper = () => {
    console.log('postmesage', this.currentUrl)
    window.ipc.postMessage(JSON.stringify({
      cmd: 'update_wallpaper', 
      url: this.currentUrl,
      base64Img: this.base64Img,
      error: 8923989,
      callback: 8923988,
    }));
  }

  downloadWallpaper = () => {
    window.ipc.postMessage(JSON.stringify({
      cmd: 'download_wallpaper', 
      url: this.currentUrl,
      base64Img: this.base64Img,
      error: 8923982,
      callback: 8923981,
    }));
  }

  copyUrl = () => {
    navigator.clipboard.writeText(this.currentUrl).then(function() {
    }, function(err) {
      console.error('Async: Could not copy text: ', err);
    });
  }
}