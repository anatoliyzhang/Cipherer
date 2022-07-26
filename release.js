
const fs = require('fs');
const config = JSON.parse(fs.readFileSync('manifest.json'));
const path_releases = 'releases';
const path_version = path_releases + '/' + config.version;
const path_name = path_version + '/' + config.name;
const path_images = path_name + '/images';

const copy_list = [
    'content-script.js',
    'images/Cipherer128.png',
    'images/Cipherer16.png',
    'images/Cipherer32.png',
    'images/Cipherer48.png',
    'images/Cipherer64.png',
    'manifest.json',
    'popup.html',
    'popup.js',
    'popup_style.css',
    'README.md',
    'wasm-glue.js',
    'wasm.js',
];
const isExistDir = (dir_name) => {
    if (!fs.existsSync(dir_name)) {
        fs.mkdirSync(dir_name);
    }
}

isExistDir(path_releases);
isExistDir(path_version);
isExistDir(path_name);
isExistDir(path_images);

copy_list.forEach((file) => {
    fs.copyFile(file, path_name + '/' + file, (err) => { if (err) console.log(err) });
});