Because I got sick of running Python every time I wanted to do anything with
`bcrypt`.

The API in the Python library exposes:
 * `bcrypt.hashpwd`
 * `bcrypt.checkpwd`

`bcrypt` has an adjustable amount of work in the hashing stage (which is
controlled by the number of rounds in `gensalt`).  As the MVP, we'll probably
just hard-code the number of rounds to the same as what the Python one gives.

So the `bcrust` tool should expose:
 * `bcrypt hashpw <password>`
 * `bcrypt checkpw <password> <hashed>` 
