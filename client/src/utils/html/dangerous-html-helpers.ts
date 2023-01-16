// This method creates a line between two elements. By doing so it seems to manipulate the DOM in a way that solid js
// does not except. This can lead to unexpected behavior.
export const drawLineBetweenElements = (
  elementId1: string,
  elementId2: string
) => {
  // TODO: Find alternative to this method.
  // Turns out hot reloading does not like this method. Disable it for now.
  return;
  // var div1 = document.getElementById(elementId1);
  // var div2 = document.getElementById(elementId2);

  // if (!div1 || !div2) {
  //   console.log("No divs found");
  //   return;
  // }

  // connect(div1, div2, "#FFF", 1);
};

function connect(
  div1: HTMLElement,
  div2: HTMLElement,
  color: string,
  thickness: number
) {
  var off1 = getOffset(div1);
  var off2 = getOffset(div2);
  // bottom right
  var x1 = off1.left + off1.width;
  var y1 = off1.top + off1.height;
  // top right
  var x2 = off2.left + off2.width;
  var y2 = off2.top;
  // distance
  var length = Math.sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1));
  // center
  var cx = (x1 + x2) / 2 - length / 2;
  var cy = (y1 + y2) / 2 - thickness / 2;
  // angle
  var angle = Math.atan2(y1 - y2, x1 - x2) * (180 / Math.PI);
  // make hr
  var htmlLine =
    "<div style='padding:0px; margin:0px; height:" +
    thickness +
    "px; background-color:" +
    color +
    "; line-height:1px; position:absolute; left:" +
    cx +
    "px; top:" +
    cy +
    "px; width:" +
    length +
    "px; -moz-transform:rotate(" +
    angle +
    "deg); -webkit-transform:rotate(" +
    angle +
    "deg); -o-transform:rotate(" +
    angle +
    "deg); -ms-transform:rotate(" +
    angle +
    "deg); transform:rotate(" +
    angle +
    "deg);' />";

  document.body.innerHTML += htmlLine;
}

function getOffset(el: HTMLElement) {
  var rect = el.getBoundingClientRect();
  return {
    left: rect.left + window.pageXOffset,
    top: rect.top + window.pageYOffset,
    width: (rect.width || el.offsetWidth) / 2,
    height: rect.height || el.offsetHeight,
  };
}
