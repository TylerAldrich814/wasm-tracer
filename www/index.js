
async function init() {
  const { instance } = await WebAssembly.instantiateStreaming(
    fetch("./wasm_tracer.wasm")
  );
  const width  = 1200;
  const height = 675;
  
  const canvas = document.getElementById("tracer-canvas");
  canvas.width = width;
  canvas.height = height;
  
  const buffer_address = instance.exports.BUFFER.value;
  const image = new ImageData(
    new Uint8ClampedArray(
      instance.exports.memory.buffer,
      buffer_address,
      4 * width * height
    ),
    width,
  );
  const ctx = canvas.getContext("2d");

  var inDev = false;
  
  
  var stop = false;
  var frameCount = 0;
  var fps, fpsInterval, startTime, now, then, elapsed;

  const render = () => {
    if( stop ){ return }
    
    instance.exports.go();
    // ctx.putImageData(image, 0, 0);
    requestAnimationFrame(render);
    
    now = Date.now();
    elapsed = now - then;
    if( elapsed > fpsInterval ){
      then = now - (elapsed % fpsInterval);

      ctx.putImageData(image, 0, 0);
    }
    
    if( inDev ){
      var sinceStart = now = startTime;
      var currentFps = Math.round(1000 / (sinceStart / ++frameCount) * 100);
      console.log(`{} FPS`, currentFps);
    }
  };

  const startAnimation = (fps) => {
    fpsInterval = 1000 / fps;
    then = Date.now();
    startTime = then;
    // console.log(startTime);

    render();
  };
  startAnimation(30);
  // instance.exports.go();        
  // ctx.putImageData(image, 0, 0);
} // end async init();

init();
