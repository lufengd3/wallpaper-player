(function () {
    'use strict';

    /******************************************************************************
    Copyright (c) Microsoft Corporation.

    Permission to use, copy, modify, and/or distribute this software for any
    purpose with or without fee is hereby granted.

    THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
    REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
    AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
    INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
    LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
    OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
    PERFORMANCE OF THIS SOFTWARE.
    ***************************************************************************** */
    /* global Reflect, Promise, SuppressedError, Symbol */

    var extendStatics = function(d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };

    function __extends(d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    }

    typeof SuppressedError === "function" ? SuppressedError : function (error, suppressed, message) {
        var e = new Error(message);
        return e.name = "SuppressedError", e.error = error, e.suppressed = suppressed, e;
    };

    var BaseComponent = (function () {
        function BaseComponent() {
            this.dom = this.renderDOM();
            this.appendCss();
            this.addEventListener();
        }
        BaseComponent.prototype.renderDOM = function () {
        };
        BaseComponent.prototype.appendCss = function () {
        };
        BaseComponent.prototype.addEventListener = function () {
        };
        return BaseComponent;
    }());

    var BackBtn = (function (_super) {
        __extends(BackBtn, _super);
        function BackBtn() {
            return _super.call(this) || this;
        }
        BackBtn.prototype.renderDOM = function () {
            var backBtn = document.createElement('div');
            backBtn.setAttribute('class', '_wpBackBtn');
            var backBtnCss = "\n      ._wpBackBtn {\n        width: 40px;\n        height: 40px;\n        position: fixed;\n        bottom: 20px;\n        left: 20px;\n        z-index: 99999;\n        border: 2px solid #2424;\n        border-radius: 40px;\n        display: flex;\n        justify-content: center;\n        align-items: center;\n      }\n      ._wpBackBtn:hover { \n        background-color: #3db4e7; \n        border-color: #fff !important; \n        cursor: pointer \n      }\n    ";
            var backArrow = document.createElement('div');
            backArrow.setAttribute('class', '_wpBackArrow');
            var backArrowCss = "\n      ._wpBackArrow {\n        width: 16px;\n        height: 16px;\n        margin-left: 6px;\n        border-width: 2px;\n        border-color: inherit;\n        border-style: solid;\n        border-right: 0;\n        border-bottom: 0;\n        transform: rotate(-45deg);\n      }\n    ";
            backBtn.appendChild(backArrow);
            document.body.appendChild(backBtn);
            var wpHhostStyle = document.createElement('style');
            wpHhostStyle.innerHTML = "".concat(backBtnCss, " ").concat(backArrowCss);
            document.body.appendChild(wpHhostStyle);
            return backBtn;
        };
        BackBtn.prototype.addEventListener = function () {
            this.dom.addEventListener('click', function () {
                history.back();
            });
        };
        return BackBtn;
    }(BaseComponent));

    var ADD_TO_QUEUE = '93984302';
    var SET_WP = '12798371231';
    var COPY = '123892x003';
    var MENU_WIDTH = '120px';
    var IMG_INFO_DOM_ID = '_wpContextMenuImgSize';
    var ContextMenu = (function () {
        function ContextMenu() {
            var _this = this;
            this.renderDOM = function () {
                var contextMenu = document.createElement('div');
                contextMenu.setAttribute('class', '_wpContextMenuContainer');
                contextMenu.addEventListener('click', _this.handleMenuClick);
                contextMenu.innerHTML = "\n    <ul class=\"_wpContextMenuBody\">\n      <li id=\"".concat(IMG_INFO_DOM_ID, "\"></li>\n      <li data-action=\"").concat(SET_WP, "\">\u8BBE\u4E3A\u58C1\u7EB8</li>\n      <li data-action=\"").concat(ADD_TO_QUEUE, "\">\u52A0\u5165\u4E0B\u8F7D\u961F\u5217</li>\n      <li data-action=\"").concat(COPY, "\">\u590D\u5236\u56FE\u7247 URL</li>\n    </ul>\n    ");
                document.body.appendChild(contextMenu);
                return contextMenu;
            };
            this.appendCss = function () {
                var contextMenuCss = "\n      ._wpContextMenuContainer {\n        width: 100vw;\n        height: 100vh;\n        position: fixed;\n        top: 0;\n        left: 0;\n        z-index: 9999999;\n        display: none;\n      }\n      ._wpContextMenuBody {\n        padding: 0 10px;\n        background-color: #f4f4f4;\n        color: #333;\n        box-shadow: 1px 1px 5px 1px rgba(0,0,0,0.2);\n        width: ".concat(MENU_WIDTH, ";\n        display: inline-flex;\n        flex-direction: column;\n        position: absolute;\n      }\n\n      #").concat(IMG_INFO_DOM_ID, " {\n        color: #999;\n      }\n\n      #").concat(IMG_INFO_DOM_ID, ":hover {\n        cursor: context-menu;\n        font-weight: normal;\n      }\n\n      ._wpContextMenuBody li {\n        height: 40px;\n        line-height: 40px;\n        text-align: left;\n        list-style: none;\n      }\n      \n      ._wpContextMenuBody li:hover {\n        cursor: pointer;\n        font-weight: bold;\n      }\n      \n      ._wpContextMenuBody li:not(:last-child) {\n        border-bottom: 0.5px solid rgba(200, 200, 200, 0.5);\n      }\n\n      ._wpContextMenuBody li:hover {\n        cursor: pointer;\n      }\n    ");
                var wpHhostStyle = document.createElement('style');
                wpHhostStyle.innerHTML = contextMenuCss;
                document.head.appendChild(wpHhostStyle);
            };
            this.addEventListener = function () {
                window.addEventListener('contextmenu', function (e) {
                    var _a = e.target, tagName = _a.tagName, src = _a.src, style = _a.style;
                    if (/img/i.test(tagName) && src) {
                        var realUrl = _this.handleImgUrl(src, e);
                        if (realUrl) {
                            e.preventDefault();
                            _this.generateImgBase64Data(e.target);
                        }
                    }
                    else if (/http/i.test(style.backgroundImage)) {
                        var url = style.backgroundImage.replace(/["']/g, "").slice(4, -1);
                        var realUrl = _this.handleImgUrl(url, e);
                        if (realUrl) {
                            e.preventDefault();
                            var img_1 = new Image();
                            img_1.src = realUrl;
                            img_1.onload = function () {
                                _this.generateImgBase64Data(img_1);
                            };
                        }
                    }
                });
                document.querySelector('._wpContextMenuBody').addEventListener('click', function (e) {
                    var dataset = e.target.dataset;
                    switch (dataset.action) {
                        case SET_WP:
                            _this.setWallpaper();
                            break;
                        case ADD_TO_QUEUE:
                            _this.downloadWallpaper();
                            break;
                        case COPY:
                            _this.copyUrl();
                            break;
                    }
                    _this.hideMenu();
                });
            };
            this.generateImgBase64Data = function (img) {
                if (img.naturalWidth >= 1920) {
                    var canvas = document.createElement("canvas");
                    canvas.width = img.naturalWidth;
                    canvas.height = img.naturalHeight;
                    var ctx = canvas.getContext("2d");
                    ctx.drawImage(img, 0, 0);
                    _this.base64Img = canvas.toDataURL("image/jpeg", 0.99);
                }
                else {
                    _this.base64Img = '';
                }
            };
            this.handleImgUrl = function (url, e) {
                if (!url) {
                    _this.currentUrl = '';
                    return;
                }
                _this.currentUrl = url.replace(/&(h|height)=\d+/g, '').replace(/&(w|width)=\d+/g, '');
                _this.showMenu(e.clientX, e.clientY);
                var imgInfoDom = document.getElementById(IMG_INFO_DOM_ID);
                if (imgInfoDom) {
                    var img = e.target;
                    var _a = img.naturalWidth, naturalWidth = _a === void 0 ? 0 : _a, _b = img.naturalHeight, naturalHeight = _b === void 0 ? 0 : _b;
                    imgInfoDom.textContent = naturalWidth ? "".concat(naturalWidth, " x ").concat(naturalHeight) : 'Unknow';
                }
                return _this.currentUrl;
            };
            this.handleMenuClick = function (e) {
                var elm = e.target;
                console.log(e.target);
                if (elm === _this.dom) {
                    _this.hideMenu();
                    return;
                }
                console.log(elm.dataset.action);
            };
            this.showMenu = function (x, y) {
                _this.dom.style.display = 'block';
                var menuBody = document.querySelector('._wpContextMenuBody');
                var menuWidth = parseInt(MENU_WIDTH);
                var menuHeight = 125;
                var docWidth = document.documentElement.offsetWidth;
                var docHeight = document.documentElement.offsetHeight;
                var left = x + menuWidth > docWidth ? x - menuWidth : x;
                var top = y + menuHeight > docHeight ? y - menuHeight : y;
                menuBody.style.left = left + 'px';
                menuBody.style.top = top + 'px';
            };
            this.hideMenu = function () {
                _this.dom.style.display = 'none';
            };
            this.setWallpaper = function () {
                console.log('postmesage', _this.currentUrl);
                window.ipc.postMessage(JSON.stringify({
                    cmd: 'update_wallpaper',
                    url: _this.currentUrl,
                    base64Img: _this.base64Img,
                    error: 8923989,
                    callback: 8923988,
                }));
            };
            this.downloadWallpaper = function () {
                window.ipc.postMessage(JSON.stringify({
                    cmd: 'download_wallpaper',
                    url: _this.currentUrl,
                    base64Img: _this.base64Img,
                    error: 8923982,
                    callback: 8923981,
                }));
            };
            this.copyUrl = function () {
                navigator.clipboard.writeText(_this.currentUrl).then(function () {
                }, function (err) {
                    console.error('Async: Could not copy text: ', err);
                });
            };
            this.dom = this.renderDOM();
            this.appendCss();
            this.addEventListener();
        }
        return ContextMenu;
    }());

    var processTagA = (function () {
        var processAnchor = function (a) {
            a.setAttribute('target', '');
        };
        var insertedObserver = new MutationObserver(function (mutations) {
            mutations.forEach(function (m) {
                var inserted = [].slice.call(m.addedNodes);
                while (inserted.length > 0) {
                    var elem = inserted.shift();
                    [].slice.call(elem.children || []).forEach(function (el) {
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
    });

    function _wp_page_init() {
        new BackBtn();
        new ContextMenu();
        processTagA();
    }
    try {
        if (!/localhost/.test(location.host)) {
            window.addEventListener('DOMContentLoaded', function () {
                _wp_page_init();
            });
        }
    }
    catch (e) {
        console.log('wp code error', e);
    }

})();
