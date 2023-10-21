export default class BaseComponent {
  dom: any;

  constructor() {
    this.dom = this.renderDOM();
    this.appendCss();
    this.addEventListener();
  }

  renderDOM() {
  }

  appendCss() {
  }

  addEventListener() {
  }
}