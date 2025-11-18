# Seccomp Profiles for RustyYOLO

This directory contains seccomp (secure computing mode) profiles that restrict which system calls the AI agent can make inside the container.

## What is Seccomp?

Seccomp is a Linux kernel feature that allows you to filter system calls a process can make. This is a critical security layer that prevents even compromised processes from performing dangerous operations.

## Why Seccomp Matters for AI Agents

Even with filesystem isolation, privilege isolation, and network firewalling, an AI agent could theoretically:
- Use `ptrace` to inspect other processes
- Attempt to load kernel modules
- Try to manipulate kernel keyrings
- Attempt namespace escapes
- Execute other privilege escalation techniques

**Seccomp blocks these syscalls at the kernel level**, providing defense-in-depth.

## Available Profiles

### 1. `seccomp-default.json` (Conservative, Recommended)

This is the **default profile** used by RustyYOLO. It's based on Docker's default seccomp profile but ensures critical dangerous syscalls are blocked.

**What it allows:**
- ✅ File I/O operations (read, write, open, close, etc.)
- ✅ Network operations (socket, connect, send, recv, etc.)
- ✅ Process management (fork, exec, wait, exit, etc.)
- ✅ Memory management (mmap, munmap, brk, etc.)
- ✅ Terminal I/O (ioctl for interactive sessions)
- ✅ Thread creation (clone with specific flags)
- ✅ Signal handling
- ✅ Time operations
- ✅ IPC mechanisms (pipes, message queues, shared memory)

**What it blocks:**
- ❌ `ptrace` - Process debugging and inspection
- ❌ `mount` / `umount` / `pivot_root` - Filesystem manipulation
- ❌ `init_module` / `finit_module` / `delete_module` - Kernel module loading
- ❌ `reboot` / `kexec_load` - System reboot/shutdown
- ❌ `bpf` - Loading eBPF programs
- ❌ `perf_event_open` - Performance monitoring that could leak info
- ❌ `keyctl` / `add_key` / `request_key` - Kernel keyring manipulation
- ❌ `iopl` / `ioperm` - Direct hardware I/O port access
- ❌ `acct` - Process accounting manipulation
- ❌ `settimeofday` / `stime` / `clock_settime` - System time manipulation
- ❌ `swapon` / `swapoff` - Swap management
- ❌ `quotactl` - Disk quota manipulation
- ❌ `lookup_dcookie` - Directory entry cache inspection
- ❌ `userfaultfd` - User-space page fault handling
- ❌ `kcmp` - Process comparison (info leak)
- ❌ `unshare` / `setns` - Namespace manipulation (when not CAP_SYS_ADMIN)
- ❌ `clone3` (without CAP_SYS_ADMIN) - Prevents new namespace creation

**Use this profile if:**
- You want strong security without breaking functionality
- You're running Claude Code in YOLO mode
- You want the "recommended" security posture

### 2. `seccomp-restrictive.json` (Maximum Security)

This is an **example restrictive profile** that provides maximum security at the potential cost of breaking some edge-case functionality.

**Additional restrictions beyond default:**
- More aggressive filtering of clone operations
- Removes some less-common syscalls that Claude Code shouldn't need
- Prioritizes security over compatibility

**What it explicitly blocks (in addition to default):**
- More restrictive `personality` filtering
- Blocks `clone3` completely (no exceptions)
- Removes NUMA memory policy syscalls (`get_mempolicy`, `set_mempolicy`, `mbind`)
- Removes some obscure architecture-specific syscalls

**Use this profile if:**
- You prioritize security over functionality
- You're willing to debug potential syscall denials
- You're testing a new AI agent and want maximum restriction

## Usage

### Using the Default Profile (Recommended)

The default profile is **automatically applied** when you run RustyYOLO:

```bash
rustyolo claude
```

No additional flags needed! The Rust CLI embeds and applies `seccomp-default.json` automatically.

### Using a Custom Profile

To use a different profile:

```bash
rustyolo --seccomp-profile /path/to/seccomp-restrictive.json claude
```

Or use the restrictive profile from this directory:

```bash
rustyolo --seccomp-profile ./seccomp/seccomp-restrictive.json claude
```

### Disabling Seccomp (Not Recommended)

To disable seccomp entirely (for debugging only):

```bash
rustyolo --seccomp-profile none claude
```

**WARNING:** This removes a critical security layer. Only use this if you're debugging syscall issues.

## Creating Custom Profiles

You can create your own seccomp profile based on your needs:

1. **Start with a template**: Copy `seccomp-default.json` or `seccomp-restrictive.json`
2. **Add/remove syscalls**: Edit the `syscalls` array
3. **Test it**: Run with `--seccomp-profile ./my-custom-profile.json`
4. **Debug denials**: Check `dmesg` for seccomp audit logs

### Seccomp Profile Format

```json
{
  "defaultAction": "SCMP_ACT_ERRNO",  // Block by default
  "defaultErrnoRet": 1,               // Return errno 1 (EPERM)
  "syscalls": [
    {
      "names": ["read", "write", "open"],  // Syscall names
      "action": "SCMP_ACT_ALLOW"            // Allow these
    }
  ]
}
```

**Actions:**
- `SCMP_ACT_ALLOW` - Allow the syscall
- `SCMP_ACT_ERRNO` - Block and return an error
- `SCMP_ACT_KILL` - Kill the process (harsh)

**Conditional rules** (advanced):
```json
{
  "names": ["clone"],
  "action": "SCMP_ACT_ALLOW",
  "args": [
    {
      "index": 0,           // Argument position
      "value": 2114060288,  // Expected value
      "op": "SCMP_CMP_MASKED_EQ"  // Comparison operator
    }
  ]
}
```

## Debugging Seccomp Denials

If the agent fails with mysterious "Operation not permitted" errors:

1. **Check kernel logs** (requires host access):
   ```bash
   sudo dmesg | grep SECCOMP
   ```

2. **Try the permissive profile**:
   ```bash
   rustyolo --seccomp-profile none claude
   ```
   If it works now, a syscall is being blocked.

3. **Add audit logging** to your profile:
   ```json
   {
     "defaultAction": "SCMP_ACT_LOG"  // Log denials instead of blocking
   }
   ```

4. **Check which syscall failed** in dmesg, then add it to your profile.

## Security Best Practices

1. **Always use a seccomp profile** - The default is there for a reason
2. **Start with default** - Only move to restrictive if you need maximum security
3. **Test before deploying** - Run your typical workflows to ensure nothing breaks
4. **Review audit logs** - Periodically check for denied syscalls
5. **Combine with other layers** - Seccomp is one layer; also use filesystem, privilege, and network isolation

## References

- [Docker Seccomp Documentation](https://docs.docker.com/engine/security/seccomp/)
- [Linux Seccomp Man Page](https://man7.org/linux/man-pages/man2/seccomp.2.html)
- [Docker Default Seccomp Profile](https://github.com/moby/moby/blob/master/profiles/seccomp/default.json)
- [Seccomp Operator Reference](https://github.com/seccomp/libseccomp/blob/main/include/seccomp.h.in)

## Blocked Syscalls Reference

Here's a comprehensive list of dangerous syscalls that are blocked by the default profile:

### Privilege Escalation
- `ptrace` - Process tracing/debugging
- `process_vm_readv` / `process_vm_writev` - Read/write other process memory

### Kernel Manipulation
- `init_module` / `finit_module` - Load kernel modules
- `delete_module` - Remove kernel modules
- `create_module` - Create kernel module (obsolete)
- `query_module` - Query module info (obsolete)

### System Control
- `reboot` - Reboot system
- `kexec_load` / `kexec_file_load` - Load new kernel for execution
- `syslog` - Kernel log manipulation
- `_sysctl` / `sysctl` - Kernel parameter manipulation

### Time Manipulation
- `settimeofday` - Set system time
- `stime` - Set system time (obsolete)
- `clock_settime` / `clock_settime64` - Set clock time
- `clock_adjtime` / `clock_adjtime64` - Adjust clock (already allowed in default for NTP)

### Filesystem Control
- `mount` - Mount filesystems
- `umount` / `umount2` - Unmount filesystems
- `pivot_root` - Change root filesystem
- `chroot` - Change root directory
- `swapon` / `swapoff` - Swap management

### Namespace Manipulation
- `unshare` - Create new namespaces (without CAP_SYS_ADMIN)
- `setns` - Join existing namespace (without CAP_SYS_ADMIN)

### Security & Auditing
- `keyctl` / `add_key` / `request_key` - Kernel keyring
- `acct` - Process accounting control

### Performance & Profiling
- `perf_event_open` - Performance monitoring (can leak sensitive info)
- `lookup_dcookie` - Directory entry cache lookup

### Hardware Access
- `iopl` - Change I/O privilege level
- `ioperm` - Set port I/O permissions
- `modify_ldt` - Modify Local Descriptor Table (allowed in default but dangerous)

### Obscure/Dangerous
- `bpf` - Load eBPF programs
- `userfaultfd` - User-space page fault handling
- `kcmp` - Compare processes (info leak)
- `quotactl` - Disk quota control
- `vm86` / `vm86old` - Enter virtual 8086 mode
- `afs_syscall` / `break` / `ftime` / `getpmsg` / `gtty` / `lock` / `mpx` / `prof` / `profil` / `putpmsg` / `security` / `stty` / `tuxcall` / `ulimit` / `vserver` - Obsolete/unimplemented syscalls

## License

These seccomp profiles are provided as-is for use with RustyYOLO. Feel free to modify and distribute.
