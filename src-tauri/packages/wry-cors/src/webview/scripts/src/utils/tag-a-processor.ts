export default () => {
  var processAnchor = function(a) {
      a.setAttribute('target', '');
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