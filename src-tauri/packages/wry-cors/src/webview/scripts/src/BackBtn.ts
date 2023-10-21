import BaseComponent from "./BaseComponent";

export default class BackBtn extends BaseComponent {
  constructor() {
    super();
  }

  renderDOM() {
    var backBtn = document.createElement('div');
    backBtn.setAttribute('class', '_wpBackBtn');
    var backBtnCss = `
      ._wpBackBtn {
        width: 40px;
        height: 40px;
        position: fixed;
        bottom: 20px;
        left: 20px;
        z-index: 99999;
        border: 2px solid #2424;
        border-radius: 40px;
        display: flex;
        justify-content: center;
        align-items: center;
      }
      ._wpBackBtn:hover { 
        background-color: #3db4e7; 
        border-color: #fff !important; 
        cursor: pointer 
      }
    `;

    var backArrow = document.createElement('div');
    backArrow.setAttribute('class', '_wpBackArrow');
    var backArrowCss = `
      ._wpBackArrow {
        width: 16px;
        height: 16px;
        margin-left: 6px;
        border-width: 2px;
        border-color: inherit;
        border-style: solid;
        border-right: 0;
        border-bottom: 0;
        transform: rotate(-45deg);
      }
    `;

    backBtn.appendChild(backArrow);
    document.body.appendChild(backBtn);

    var wpHhostStyle = document.createElement('style');
    wpHhostStyle.innerHTML = `${backBtnCss} ${backArrowCss}`;
    document.body.appendChild(wpHhostStyle);

    return backBtn;
  }

  addEventListener(): void {
    this.dom.addEventListener('click', () => {
      history.back();
    });
  }

}