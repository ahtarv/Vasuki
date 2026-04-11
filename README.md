# Vasuki 
> A portable microVM manager and container runtime, written in Rust from scratch.

Vasuki combines three things: a **RISC-V CPU emulator** that can boot a real kernel, a **hypervisor layer** that accelerates it with WHPX (Windows) or KVM (Linux), and a **container runtime** that spins up microVMs instead of plain namespaces. Same idea as AWS Firecracker — built by hand, from zero.

---

## Architecture

```
vasuki/
├── crates/
│   ├── vm-core/       # RISC-V RV64GC emulator — fetch, decode, execute
│   ├── hypervisor/    # WHPX (Windows) + KVM (Linux) abstraction
│   ├── memory/        # Guest memory, page tables, MMU
│   ├── devices/       # Virtio serial, block, network
│   ├── runtime/       # OCI image pulling, container lifecycle
│   └── cli/           # vasuki run / ps / exec
```

---

## Roadmap

- [ ] **Phase 1 — RISC-V base ISA** — RV64I, registers, memory bus, run a hand-assembled binary
- [ ] **Phase 2 — Full ISA + boot** — RV64GC extensions, UART device, boot a Linux initramfs
- [ ] **Phase 3 — Hypervisor layer** — WHPX on Windows, KVM on Linux, single `Vcpu` trait
- [ ] **Phase 4 — Virtio devices** — serial, block, network. Boot real Alpine Linux inside Vasuki
- [ ] **Phase 5 — Container runtime** — OCI image support, `vasuki run alpine sh`
- [ ] **Phase 6 — Networking** — TAP devices, bridge networking, NAT, container-to-container

---

## Inspiration

- [Firecracker](https://github.com/firecracker-microvm/firecracker) — AWS microVM manager powering Lambda
- [cloud-hypervisor](https://github.com/cloud-hypervisor/cloud-hypervisor) — Intel's production Rust hypervisor
- [crosvm](https://github.com/google/crosvm) — Google's ChromeOS VM manager
- [rvemu](https://github.com/d0iasm/rvemu) — RISC-V emulator in Rust
- [youki](https://github.com/containers/youki) — OCI container runtime in Rust

---

## Status
Early development. Breaking changes everywhere. Do not use in production.

## License
MIT
