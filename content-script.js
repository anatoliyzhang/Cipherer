// How to detect a piece of text wheather it is encrypted by this programme and can be decrypted?
// There should be some special traits, which can be detected(, but not easily spoted by human eyes) then send to decrypt.
// in this program I chose a rule: if the text matches this pattern, then I think this text IS the encrypted text, and can
// be decrypted by decrypt function, which is in this case the wasm.
// This pattern is : /.* _SPLITER_ (.*) _SPLITER_.*/g, which means if the text contains the string _SPLITER_ twice, then this text IS the encryted
// I'm looking for. Here I take '0Z' for the _SPLITER_.
// Actually, the string before and after the _SPLITER_ can be ignored, only the text between _SPLITER_ is needed to decrypt.
// Why I take "0Z"? Because the function randomString(leng) in the popup.js can not generate a string, which contains '0' and the wasm in the background
// only generates an encrypted text, which does not include any of "0189", because the algrithm is BASE32 with alphabet RFC4648.
// see the source code in lib.rs const RFC4648_ALPHABET: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
// //So, TODO: what if a normal text like "abc0Zgoodmorining0Z0Z0Z" will happen?
const spliter = '0Z';
const public_key = "public_key";
// find out the encrypted text/values and replace them with the decrypted texts/values:
// codes of function replaeInNodes inspired from https://stackoverflow.com/questions/5558613/replace-words-in-the-body-text
// retrives all childNodes of body
let childNodes = document.body.childNodes;
// const pattern = new RegExp('')  
async function replaceInNodes(nodes) {
    // const pattern = /%OT(.*)%KO/g;
    // iterate through all nodes

    for (let i = 0; i < nodes.length; i++) {
        let curNode = nodes[i];
        // if the node has attributes, let us look at those
        // i.E. we want to change "John" in the input placeholder to "Peter" - <input type="text" value="John">
        if (curNode.attributes !== undefined) {
            let curNodeAttributes = curNode.attributes;
            for (let j = 0; j < curNodeAttributes.length; j++) {
                // find out the encrypted text
                let encrypted_text = curNodeAttributes[j].nodeValue.split(spliter);
                if (encrypted_text.length === 3) {

                    try {
                        curNodeAttributes[j].nodeValue = decrypto(encrypted_text[1], public_key);
                    } catch (e) { console.log(e) }
                }
            }
        }
        // It is a "TEXT_NODE"
        // i.E. <span>John</span>
        if (curNode.nodeType == 3) {
            let encrypted_text = curNode.data.split(spliter);
            if (encrypted_text.length === 3) {
                try {
                    curNode.data = encrypted_text[0].slice(0, -16) + decrypto(encrypted_text[1], public_key)
                        + encrypted_text[2].slice(16);
                } catch (e) { console.log(e) }
            }
        }
        // It is a "ELEMENT_NODE", meaning we need to go deeper
        if (curNode.nodeType == 1) {
            this.replaceInNodes(curNode.childNodes);
        }
    }
}

(async () => {
    await init(wasm_string);
    replaceInNodes(childNodes);
})();

