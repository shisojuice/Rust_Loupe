import init, { zoom_img,resize_img } from './rust_loupe.js';

const mainImg =  document.getElementById("mainImg");
const rect = mainImg.getBoundingClientRect();
const cursor = document.getElementById('my_cursor');

async function run() {
    await init();

    document.getElementById("file_input").addEventListener("change", async (event) => {
        const files = document.getElementById("file_input").files;
        if (files.length === 0) {
            return;
        }
        const file_blob = new Blob([files[0]], { type: files[0].type });
        await blobToUint8Array(file_blob)
            .then(uint8Array => {
                mainImg.src  = resize_img(uint8Array);
            })
            .catch(error => {
                console.error('Error converting blob:', error);
            });
    });

    document.getElementById("mainImg").addEventListener("mousemove", async (event) => {
        cursor.style.left = event.clientX > mainImg.offsetWidth  + rect.left - cursor.offsetWidth -2 ?(mainImg.offsetWidth  + rect.left - cursor.offsetWidth -2)+"px" : event.clientX + 'px' ;
        cursor.style.top = event.clientY > mainImg.offsetHeight  + rect.top - cursor.offsetHeight -2 ?(mainImg.offsetHeight  + rect.top - cursor.offsetHeight -2)+"px" : event.clientY + 'px' ;
        const files = document.getElementById("file_input").files;
        if (files.length === 0) {
            return;
        }
        document.getElementById("bigImg").src  = zoom_img(mainImg.src,event.offsetX,event.offsetY);
    });
    document.getElementById("mainImg").addEventListener("touchmove", async (event) => {
        const touch = event.touches[0];
        cursor.style.left = touch.clientX > mainImg.offsetWidth  + rect.left - cursor.offsetWidth -2 ?(mainImg.offsetWidth  + rect.left - cursor.offsetWidth -2)+"px" : touch.clientX + 'px' ;
        cursor.style.top = touch.clientY > mainImg.offsetHeight  + rect.top - cursor.offsetHeight -2 ?(mainImg.offsetHeight  + rect.top - cursor.offsetHeight -2)+"px" : touch.clientY + 'px' ;
        const files = document.getElementById("file_input").files;
        if (files.length === 0) {
            return;
        }
        document.getElementById("bigImg").src  = zoom_img(mainImg.src,touch.offsetX,touch.offsetY);
    });
}
run();

async function blobToUint8Array(blob) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => {
            resolve(new Uint8Array(reader.result));
        };
        reader.onerror = reject;
        reader.readAsArrayBuffer(blob);
    });
}