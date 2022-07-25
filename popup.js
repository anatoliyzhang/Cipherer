let original_text_textarea = document.getElementById('original_text_textarea');
let change_text_button = document.getElementById('encrypt_text_button');
let decrypt_text_button = document.getElementById('decrypt_text_button');
let result_type_div = document.getElementById('result_type_div');
let copy_status = document.getElementById('copy_status');
let result_text_div = document.getElementById('result_text_div');
const fromHex = hexString => new Uint8Array(hexString.match(/.{1,2}/g).map(byte => parseInt(byte, 16)));
const fromBase64 = base64String => Uint8Array.from(atob(base64String), c => c.charCodeAt(0));

const traits = {pre: '',start:'%OT', end:'%KO', suf: ''};
const spliter = '0Z';

function randomString(leng) {
  // leng = Math.ceil(Math.random()*leng);
  leng = leng || 16;
  var $chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ123456789';
  var maxPos = $chars.length;
  var pwd = '';
  for (i = 0; i < leng; i++) {
  pwd += $chars.charAt(Math.floor(Math.random() * maxPos));
  }
  return pwd;
  }

(async ()=>{
  await init(wasm_string);
    change_text_button.onclick = async ()=>{
      let splited_text = original_text_textarea.value.split(spliter);
      result_type_div.innerText = "Result of Encrypting";
      if (splited_text.length >=3) {
        result_text_div.innerText = original_text_textarea.value;

        return;
      }
      let random_text_start = randomString(16);
      let random_text_end = randomString(16);
      const encrypted_text = encrypto(original_text_textarea.value, "public_key");
      result_text_div.innerText = random_text_start + spliter + encrypted_text + spliter + random_text_end;
       // + traits.end + random_text_end;
    };

    decrypt_text_button.onclick = async ()=>{
      let ciphered_text = original_text_textarea.value.split(spliter);
      result_type_div.innerText = "result of Decrypting:";
      if (ciphered_text <3) {
        result_text_div.innerText = original_text_textarea.value;
        return;
      }
      const decrypted = decrypto(ciphered_text[1], "public_key");
      result_text_div.innerText = decrypted;
    };

    result_text_div.onclick = ()=>{
      navigator.clipboard.writeText(result_text_div.innerText);
      copy_status.innerText = result_type_div.innerText +" is copied to Clipboard.";
    };

    // console.log(encodeURIComponent('u/Kn'));
    // console.log(rawKey);
    // console.log(counter);
})();