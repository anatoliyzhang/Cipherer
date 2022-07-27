# Cipherer
This is a chrome extension, which does 2 jobs:
1. Encrypts normal text to a ciphertext. 
2. With this chrome extension enabled, automatically finds the encrypted text and decrypts it to normal text.
The goal of creation of this chrome extension is to bypass the internet censorship by posting the ciphertext, which is harder to be censored.

# How does it work?
The extension sends the normal text to a wasm application (blackbox) to encrypt or decrypt.The blackbox itself is written in Rust,some very simple codes,then compiled to Webassembly(I have already done that.). I chose wasm rather than js because wasm is harder to be cracked while javascript displays all its source code.

# How to use
1. Install nodejs.
2. Run node release.js to generate the unpacked extension package.
3. In Chome's extension manager enable developer mode and load the unpacked extension folder in __FOLDER__/releases/__VERSION__/Cipherer

# Story
From about 2000, instant chatting became popular, and I was really interested in using some programs to chat with friends, but I gradually found that my friends were using broken words to chat with me, which made me unhappy. I felt that they were not well educated, and they did not respect me. I decided to keep distance with those friends, until one day, I found that I,myself, had to use a broken word to get my article published by a site, otherwise if I would use the normal word, my essay would be deleted.My face suddenly flushed, and I realized what a terrible mistake I had made! I misunderstood those friends, and I lost them.I felt ashamed, and sorry.
Gradually gradually I got to know,that it is call censorship. There are computers and personals behind the text, they are reviewing  these texts every millisecond.They decide whether the texts can be published.
I know it is controversial whether censorship  is the right thing, but I really dislike it.
So...I have a dream.
I hope some ppl can help ppl out of the terra of censorship. 
One day the I saw a website announced that  they would not allow even using broken words ...
This is why my approach is here.
