import init, {Coords, resize_image, warp_image, get_dimensions} from './wasm/wasm.js';

const canvas = new fabric.Canvas('ripper', { selection: false });

document.getElementById('ripper').style.visibility = "hidden";
document.getElementById('rip-texture-button').style.visibility = "hidden";
document.getElementById('resize-texture-button').style.visibility = "hidden";

fabric.Object.prototype.originX = fabric.Object.prototype.originY = 'center';

function makeCircle(left, top, line1, line2) {
  var c = new fabric.Circle({
    left: left,
    top: top,
    strokeWidth: 2,
    radius: 6,
    fill: '#292929',
    stroke: '#eeeeec'
  });
  c.hasControls = c.hasBorders = false;

  c.line1 = line1;
  c.line2 = line2;

  return c;
}

function makeLine(coords) {
  return new fabric.Line(coords, {
    stroke: '#eeeeec',
    strokeWidth: 2,
    selectable: false,
    evented: false,
  });
}

const canvasObjects = [];
let topLeftCircle;
let topRightCircle;
let bottomLeftCircle;
let bottomRightCircle;

function addToCanvas(fabricObject) {
  canvas.add(fabricObject);
  canvasObjects.push(fabricObject);
}

function clearCanvasObjects() {
  canvasObjects.forEach(function (canvasObject) {
    canvas.remove(canvasObject);
  });
}

function initializeControls(width, height) {
  clearCanvasObjects();

  var topHeight = Math.floor(height / 4);
  var bottomHeight = Math.floor((height / 4) * 3);
  var rightWidth = Math.floor((width / 4) * 3);
  var leftWidth = Math.floor(width / 4);

  const topLine = makeLine([leftWidth, topHeight, rightWidth, topHeight]);
  const rightLine = makeLine([rightWidth, topHeight, rightWidth, bottomHeight]);
  const bottomLine = makeLine([rightWidth, bottomHeight, leftWidth, bottomHeight]);
  const leftLine = makeLine([leftWidth, bottomHeight, leftWidth, topHeight]);

  addToCanvas(topLine);
  addToCanvas(leftLine);
  addToCanvas(rightLine);
  addToCanvas(bottomLine);

  topLeftCircle = makeCircle(topLine.get('x1'), topLine.get('y1'), topLine, leftLine);
  topRightCircle = makeCircle(topLine.get('x2'), topLine.get('y2'), rightLine, topLine);
  bottomLeftCircle = makeCircle(bottomLine.get('x2'), bottomLine.get('y2'), leftLine, bottomLine);
  bottomRightCircle = makeCircle(bottomLine.get('x1'), bottomLine.get('y1'), bottomLine, rightLine);

  addToCanvas(topLeftCircle);
  addToCanvas(topRightCircle);
  addToCanvas(bottomLeftCircle);
  addToCanvas(bottomRightCircle);

  canvas.renderAll();
}

canvas.on('object:moving', function(e) {
  var p = e.target;
  p.line1 && p.line1.set({
    x1: p.left,
    y1: p.top,
  });

  p.line2 && p.line2.set({
    x2: p.left,
    y2: p.top,
  });

  canvas.renderAll();
});

function coords(circle) {
  return new Coords(circle.left, circle.top);
}

let imageBytes = null;

(async () => {
  await init();

  initializeControls(canvas.width, canvas.height);

  const button = document.getElementById('button');

  let resolution = null;

  button.addEventListener('input', (event) => {
    document.getElementById('ripper').style.visibility = "visible";
    document.getElementById('rip-texture-button').style.visibility = "visible";

    let file = event.target.files[0];

    const reader = new FileReader();

    reader.readAsArrayBuffer(file);

    reader.onloadend = function() {
      imageBytes = new Uint8Array(reader.result);

      const file = new File([imageBytes.buffer], "output.png", {type: 'image/png'});

      resolution = get_dimensions(imageBytes);
      fabric.Image.fromURL(URL.createObjectURL(file), function (image) {
        canvas.setDimensions({width: resolution.width, height: resolution.height});
        canvas.setBackgroundImage(image, function () { initializeControls(resolution.width, resolution.height) }, {
          originX: 'left',
          originY: 'top'
        });
      });
    };
  });

  let rippedImage = null;

  const ripTextureButton = document.getElementById('rip-texture-button');
  ripTextureButton.onclick = (event) => {
    if (!imageBytes) {
      alert("Please select an image first.");
      return;
    }
    
    rippedImage = warp_image(imageBytes,
      coords(topLeftCircle),
      coords(topRightCircle),
      coords(bottomLeftCircle),
      coords(bottomRightCircle));

    const file = new File([rippedImage.buffer], "preview.png", {type: 'image/png'});
    const rippedImageUrl = URL.createObjectURL(file)
    document.getElementById('preview').src = rippedImageUrl;
    document.getElementById('resize-texture-button').style.visibility = "visible";
  };

  const resizeTextureButton = document.getElementById('resize-texture-button');
  resizeTextureButton.onclick = (event) => {

    const newX = resolution.x - 100;
    const newY = resolution.y - 100;

    let resizedImage = resize_image(rippedImage, newX, newY);
    const file = new File([rippedImage.buffer], "preview.png", {type: 'image/png'});
    const rippedImageUrl = URL.createObjectURL(file)
    document.getElementById('preview').src = rippedImageUrl;    
  };
})();
