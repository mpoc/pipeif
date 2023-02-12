import { readAll } from 'https://deno.land/std@0.177.0/streams/read_all.ts'
import {
    crypto,
    DigestAlgorithm,
} from 'https://deno.land/std@0.177.0/crypto/mod.ts'

const digest = async (data: Uint8Array, algorithm: DigestAlgorithm) => {
    const hash = await crypto.subtle.digest(algorithm, data)
    return Array.from(new Uint8Array(hash))
        .map((b) => b.toString(16).padStart(2, '0'))
        .join('')
}

if (import.meta.main) {
    const stdin = await readAll(Deno.stdin)

    const algorithm = Deno.args[0] as DigestAlgorithm
    const expectedHash = Deno.args[1]

    const stdinHash = await digest(stdin, algorithm)

    if (stdinHash !== expectedHash) {
        console.error(`stdin hash mismatch: ${stdinHash} !== ${expectedHash}`)
        Deno.exit(1)
    }

    await Deno.stdout.write(stdin)
    Deno.exit(0)
}
