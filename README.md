# Brainstorm (under development)

A virtual machine for server-side Brainfuck language

Rules:<br />
The last cell in the memory (by default, at index 29,999) is the flag cell<br />
Flag cell content:

0 -> "." operator writes the active port, while the "," operator reads the active port and stops its execution<br />
1 -> "." operator sends the response using the memory cells as buffer, while the "," operator reads from the request <br />
2 -> "." operator writes into standard console, while the "," operator reads from standard console<br /><br />

This processor extends from the brainfuck processor that I had made.
