#!/usr/bin/env python3
"""
Patches the compiled release ELF binary to lower GLIBC requirements.
Clears versioned symbol bindings for symbols that bound to bleeding-edge GLIBC (2.39/2.43),
and prunes the ELF .gnu.version_r table to ensure compatibility with standard Linux distributions (Ubuntu 22.04+, Debian 12+, Arch, etc.).
"""
import sys
import struct
import subprocess

def patch_binary(binary_path: str):
    # 1. Clear bleeding edge symbol versions with patchelf
    symbols_to_clear = [
        'acosf', 'atan2f', 'hypotf',
        'pidfd_getpid', 'pidfd_spawnp', 'posix_spawn_file_actions_addchdir_np'
    ]
    for s in symbols_to_clear:
        subprocess.run(['patchelf', '--clear-symbol-version', s, binary_path], check=False)

    with open(binary_path, 'r+b') as f:
        data = bytearray(f.read())
        
        e_shoff = struct.unpack_from('<Q', data, 40)[0]
        e_shentsize = struct.unpack_from('<H', data, 58)[0]
        e_shnum = struct.unpack_from('<H', data, 60)[0]
        e_shstrndx = struct.unpack_from('<H', data, 62)[0]
        shstr_sh = data[e_shoff + e_shstrndx*e_shentsize : e_shoff + (e_shstrndx+1)*e_shentsize]
        shstr_offset = struct.unpack_from('<Q', shstr_sh, 24)[0]

        for i in range(e_shnum):
            sh = data[e_shoff + i*e_shentsize : e_shoff + (i+1)*e_shentsize]
            sh_name, sh_type, sh_flags, sh_addr, sh_offset, sh_size = struct.unpack_from('<IIQQQQ', sh, 0)
            name = data[shstr_offset + sh_name:].split(b'\x00')[0].decode()
            if sh_type == 0x6ffffffe: # SHT_GNU_verneed
                dynstr_sh = data[e_shoff + struct.unpack_from('<I', sh, 40)[0] * e_shentsize:]
                dynstr_offset = struct.unpack_from('<Q', dynstr_sh, 24)[0]
                
                vn_off = sh_offset
                while True:
                    vn_version, vn_cnt, vn_file, vn_aux, vn_next = struct.unpack_from('<HHIII', data, vn_off)
                    filename = data[dynstr_offset + vn_file:].split(b'\x00')[0].decode()
                    
                    vna_off = vn_off + vn_aux
                    vna_list = []
                    for _ in range(vn_cnt):
                        vna_hash, vna_flags, vna_other, vna_name, vna_next = struct.unpack_from('<IHHI I', data, vna_off)
                        vername = data[dynstr_offset + vna_name:].split(b'\x00')[0].decode()
                        vna_list.append((vna_off, vername, vna_next))
                        if vna_next == 0:
                            break
                        vna_off += vna_next
                    
                    if filename == 'libm.so.6':
                        # Limit to GLIBC_2.29
                        target_ver = 'GLIBC_2.29'
                        new_cnt = 0
                        for idx, (offset, vername, _) in enumerate(vna_list):
                            new_cnt += 1
                            if vername == target_ver:
                                struct.pack_into('<I', data, offset + 12, 0)
                                break
                        struct.pack_into('<H', data, vn_off + 2, new_cnt)
                        print(f"Patched {filename}: pruned to {target_ver}")
                    
                    elif filename == 'libc.so.6':
                        # Limit to GLIBC_2.34
                        target_ver = 'GLIBC_2.34'
                        new_cnt = 0
                        for idx, (offset, vername, _) in enumerate(vna_list):
                            new_cnt += 1
                            if vername == target_ver:
                                struct.pack_into('<I', data, offset + 12, 0)
                                break
                        struct.pack_into('<H', data, vn_off + 2, new_cnt)
                        print(f"Patched {filename}: pruned to {target_ver}")
                    
                    if vn_next == 0:
                        break
                    vn_off += vn_next
                    
        f.seek(0)
        f.write(data)

if __name__ == '__main__':
    target = sys.argv[1] if len(sys.argv) > 1 else 'target/release/driftwm-settings'
    patch_binary(target)
