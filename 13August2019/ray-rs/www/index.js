import * as wasm from "ray-rs";

const time_text = document.getElementById('text')
const canvas = document.getElementById('canvas')

const ctx = canvas.getContext('2d')
const width = canvas.width
const height = canvas.height

var js_render_test = () => {
    let img = ctx.getImageData(0, 0, width, height)
    let buf = new ArrayBuffer(img.data.length)
    let buf8 = new Uint8ClampedArray(buf)
    for (let y = 0; y < height; y += 1) {
        for (let x = 0; x < width; x += 1) {
            let i = ((y * height) + x) * 4
            buf8[i + 0] = x % 255
            buf8[i + 1] = y % 255
            buf8[i + 2] = 128
            buf8[i + 3] = 255
        }
    }
    img.data.set(buf8)
    ctx.putImageData(img, 0, 0)
    time_text.textContent = "JS"
}

var js_render = () => {
    js_render_test()
}

var rs_render = () => {
    let img = ctx.getImageData(0, 0, width, height)
    let buf = wasm.render()
    img.data.set(buf)
    ctx.putImageData(img, 0, 0)
    time_text.textContent = "WASM"
}

var js_btn = document.getElementById('jsBtn')
var rs_btn = document.getElementById('rsBtn')
js_btn.addEventListener('click', js_render)
rs_btn.addEventListener('click', rs_render)