# Cipherer
This is a chrome extension, which does 2 jobs:
1. Encrypts normal text to a ciphertext. 
2. With this chrome extension enabled, the encrypted text will automatically be dectypted to normal text(human readable).
The goal of creation of this chrome extension is to bypass the internet censorship by posting the ciphertext, which is harder to be censored.

# How does it work?
The extension sends the normal text to a wasm application (blackbox) to encrypt or decrypt.The blackbox itself is written in Rust,some very simple codes,then compiled to Webassembly(I have already done that.). I chose wasm rather than js because wasm is harder to be cracked while javascript displays all its source code.

# How to use
1. Install nodejs.
2. Run node release.js to generate the unpacked extension package.
3. In Chome's extension manager enable developer mode and load the unpacked extension folder in __FOLDER__/releases/__VERSION__/Cipherer

# Story
From about 2000, instant chatting became popular, and I was really interested in using some programs to chat with friends, but I gradually found that some of my friends were using broken words to chat with me, which made me unhappy, because those broken words made it harder to read, and I did not get used to it. I felt that they were not well educated, and they did not respect me. I decided to keep my distance from those friends.

Until one day, I found that I, myself, had to use broken words in order to get my article published on that site, otherwise, if I would use the normal word, my essay would be deleted, because of some sensitive words, which I don't think they are, but the site does. My face suddenly flushed, and I realized what a terrible mistake I had made! I misunderstood those friends, and I lost them. I felt ashamed and sorry.

Gradually, gradually I got to know, that it is called censorship. There are computers and personals behind the text, they are reviewing these texts every millisecond. They decide whether the texts can be published. I know it is controversial whether censorship is the right thing, but I really dislike it. I hope some people can help others out of the fear of censorship. One day I saw a website announced that they would not even allow using broken words to bypass censorship...

I refuse to use broken words, I refuse to be censored, I refuse to be enforced. I HAVE A DREAM.

This is why my approach is here, a Chrome extension.

I completed this project in 2 days, in Javascript. But re-designed it in  7 days, with the back-end ciphering logic in Javascript changed to WebAssembly wasm, which is coded in Rust programming language, and I hope it would be harder to crack.

With this extension enabled, a popup page would be displayed, where a ciphertext can be generated from the original text, and conversely the ciphertext can be decrypted into the original text.

Then if you publish the ciphertext to a website, your reader would see the original (decrypted) text, if he or she has also installed the extension. But the website only gets the ciphertext, and it can not understand it, so it is much harder to censor your text.

I made it open source at https://github.com/anatoliyzhang/Cipherer
