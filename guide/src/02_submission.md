# Submission

To submit your solution, please create a fork of this repository and submit a pull request to the `main` branch. Your pull request should include the following:

- A new directory in the `solutions` directory named after your GitHub username.
- This should contain a binary rust project with your solution.
- The binary should accept a single argument, that is the file path to the input file.

For example, if your GitHub username is `sassman`, your solution should be in `solutions/sassman`.
It should be callable like this:

```sh
cd solutions/sassman
cargo run --release -- ../../samples/weather_100.csv
```

The output should be written to stdout.

Please note that CI will run on your pull request to ensure that your solution compiles and runs correctly.

:NOTE: If you have any questions, feel free to ask in the [`#hack-evening-2024-4` channel on the Rust Munich Discord server](https://discord.com/channels/704612189532586014/1315802468868817007).

Good luck! 🦀

## CI is your friend

- Your PR is automatically checked by GitHub Actions
- Benchmarks are exectued and results are provided as Comment on the PR, you can use this as a tool
- Flamegraphs are also produced during the build, they should appear on the PR as comment as well

## Submission Evaluation

We are evaluating submissions by benchmarking them on a set of three self-hosted Github runners with
dedicated vCPU cores. The servers are AMD EPYC instances (CCX33) from
[Hetzner](https://www.hetzner.com/cloud/) with the following specs:

| resource             | value |
| -------------------- | ----- |
| dedicated vCPU cores | 8     |
| RAM in GB            | 32    |
| disk space in GB     | 240   |

You submission will be terminated after 8 minutes (including build time).

```text
$ cat /proc/cpuinfo # with just the last core shown
processor       : 7
vendor_id       : AuthenticAMD
cpu family      : 25
model           : 1
model name      : AMD EPYC Processor
stepping        : 1
microcode       : 0x1000065
cpu MHz         : 2396.400
cache size      : 512 KB
physical id     : 0
siblings        : 8
core id         : 3
cpu cores       : 4
apicid          : 7
initial apicid  : 7
fpu             : yes
fpu_exception   : yes
cpuid level     : 13
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm rep_good nopl cpuid extd_apicid tsc_known_freq pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 x2apic movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext perfctr_core invpcid_single ssbd ibrs ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 erms invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr wbnoinvd arat umip pku ospke rdpid fsrm
bugs            : sysret_ss_attrs null_seg spectre_v1 spectre_v2 spec_store_bypass srso
bogomips        : 4792.80
TLB size        : 1024 4K pages
clflush size    : 64
cache_alignment : 64
address sizes   : 40 bits physical, 48 bits virtual
power management:
```
