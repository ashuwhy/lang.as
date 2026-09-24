# The Two-Language Problem, GPU/Heterogeneous Programming, and Performance-Oriented Language Efforts (2023–2026)

> Research notes compiled 2026-09-24. Method note for the report writer: the web-search budget ran out partway through, and direct page fetches were blocked for most domains except github.com. As a result, many findings below come from search-engine summaries of the cited pages, not from reading the full pages. Treat exact numbers from secondary aggregators (marked "secondary") with caution. Items before 2024 are marked **[background]**.

---

## 1. The two-language problem in practice (Python + C/C++/CUDA/Rust), what it costs, and how Julia tried to solve it

### Takeaway
The "Python on top, native code underneath" pattern is winning and still spreading. The native layer is moving toward Rust (PyO3/maturin), and NVIDIA is pouring money into Python-native CUDA tooling. The costs show up less in writing code and more in **building, packaging and distributing native/GPU binaries** (PEP 817 is the latest evidence), in debugging across the language boundary, and in having two skill sets. Julia got the one-language model mostly right technically. It lost the broad ML market on latency, correctness and composability trust, and ecosystem gravity. It kept real niches in scientific and pharma modeling.

### Cited Findings
**How the two-language stack is used today**
- The JetBrains/PSF State of Python 2025 survey had 30,000+ respondents from nearly 200 countries. 51% of Python developers do data exploration and processing, mostly with pandas and NumPy. Half of respondents had used Python for less than two years. — [JetBrains blog, Aug 2025](https://blog.jetbrains.com/pycharm/2025/08/the-state-of-python-2025/)
- Among Python developers who write binary extensions, the share using Rust rose from 27% to 33% in one year (State of Python 2025). — [The New Stack](https://thenewstack.io/rust-pythons-new-performance-engine/); [heise](https://www.heise.de/en/news/Rust-becomes-Python-s-performance-co-pilot-10560471.html)
- At the 2025 Python Language Summit, core developers said "somewhere between one-quarter and one-third of all native code being uploaded to PyPI for new projects uses Rust." — [The New Stack](https://thenewstack.io/rust-pythons-new-performance-engine/); [PSF: Language Summit 2025, "What do core developers want from Rust?"](https://pyfound.blogspot.com/2025/06/python-language-summit-2025-what-do-core-developers-want-from-rust.html?m=1). There is also a discussion of eventually allowing Rust inside CPython itself. — [LWN "Eventual Rust in CPython"](https://lwn.net/Articles/1046933/)
- Flagship Python packages now have Rust cores built with PyO3/maturin: Polars, Ruff, Pydantic v2, Hugging Face tokenizers, orjson. Maturin builds wheels for Python 3.8+ on Windows, Linux, macOS and FreeBSD. — [maturin GitHub](https://github.com/PyO3/maturin); [secondary: Nandann guide](https://www.nandann.com/blog/rust-pyo3-python-extensions-guide)
- NVIDIA is building a Python-first CUDA stack:
  - `cuda.bindings` gives full coverage of the CUDA host APIs (Driver, Runtime, NVRTC, nvJitLink, NVVM, and others).
  - `cuda.core` gives "idiomatic, Pythonic access."
  - `cuda.compute` exposes CCCL parallel algorithms.
  - `cuda.tile` provides the cuTile DSL.
  - The stated goals are to "flatten the learning curve," enable "end-to-end Python-based CUDA development," and "reduce maintenance burden for GPU library developers." — [NVIDIA/cuda-python GitHub](https://github.com/NVIDIA/cuda-python)
- NVIDIA now maintains Numba's CUDA target (numba-cuda) and has put it in **maintenance mode**: only security and critical fixes "through the lifetime of CUDA 13," with users told to move to Numba-CUDA-MLIR. — [NVIDIA/numba-cuda GitHub](https://github.com/NVIDIA/numba-cuda)
- PyTorch 2's `torch.compile` (TorchDynamo plus TorchInductor, ASPLOS '24) rewrites Python bytecode to extract FX graphs, then lowers them to Triton on GPUs and C++ on CPUs. It reports a 2.27× inference and 1.41× training geometric-mean speedup on an A100 across 180+ real models. So the dominant ML framework now contains a compiler that generates code in a *third* language (Triton) from the first (Python). — [ACM DL](https://dl.acm.org/doi/10.1145/3620665.3640366); [PyTorch blog](https://pytorch.org/blog/pytorch-pytorch-2-paper-tutorial/)

**What it costs: packaging and distribution (the most concrete 2025 evidence)**
- PEP 817, "Wheel Variants: Beyond Platform Tags," is a Draft created 10 Dec 2025 by Dekhtiar, Górny, Schütze, Gommers and six others. It documents GPU packaging pain:
  - PyTorch needs a separate index URL per accelerator plus local version tags (`+cu130`, `+rocm6.4`, `+cpu`).
  - On 2025-12-05, **552 of 8,136 issues (6.8%) on uv's issue tracker contained "torch."**
  - XGBoost ships separate `xgboost` and `xgboost-cpu` packages.
  - **CuPy has registered 55 package names** (`cupy-cuda70` … `cupy-rocm-7-1`).
  - "Fat" wheels that bundle every variant exceed PyPI size limits.
  - NumPy's runtime CPU dispatch leaves most code unable to use compiler optimizations. GROMACS AVX-512 builds are ~70% faster than generic SSE2 builds. — [PEP 817 on GitHub](https://github.com/python/peps/blob/main/peps/pep-0817.rst)

**Julia's attempt**
- **[background, 2022]** Yuri Vishnevsky's essay "Why I no longer recommend Julia" concludes that "there are too many correctness and composability bugs throughout the ecosystem to justify using it in most contexts where correctness matters." Examples:
  - Distances.jl's Euclidean distance fails with Unitful vectors.
  - Running external commands fails with substrings.
  - `missing` breaks matrix multiplication in some cases.
  - `@distributed` does not work with OffsetArrays.
  - Composing packages from multiple sources was a major bug source. — [yuri.is/not-julia](https://yuri.is/not-julia/); long community response thread (300+ posts) at [Julia Discourse](https://discourse.julialang.org/t/discussion-on-why-i-no-longer-recommend-julia-by-yuri-vishnevsky/81151)
- Julia 1.12 (Oct 2025) added experimental **code trimming** to produce small standalone binaries through JuliaC.jl. — [Julia 1.12 highlights](https://julialang.org/blog/2025/10/julia-1.12-highlights/)
- Julia 1.13 (Sept 2026) focuses on latency, "one of Julia's long-standing pain points":
  - Dedicated TTFX (time-to-first-plot/execution) CI jobs run on every PR and every master commit.
  - juliac trimming has graduated into the JuliaC.jl package and supports more constructs (finalizers, `@cfunction`, `mapreduce`).
  - Precompilation, startup and GC are faster. — [Julia 1.13 highlights](https://julialang.org/blog/2026/09/julia-1.13-highlights/); [AlternativeTo news](https://alternativeto.net/news/2026/9/julia-1-13-released-with-major-speed-gains-in-precompilation-startup-and-garbage-collection/)
- A practitioner review found trimmed Julia binaries start in ~25 ms. — [viralinstruction "Julia trimming for Advent of Code 2025"](https://viralinstruction.com/posts/aoc2025/)
- Adoption claims:
  - Julia has "more than 1 million users." — [MIT News, Aug 31 2026](https://news.mit.edu/2026/how-mit-research-project-became-global-programming-language-0831)
  - JuliaHub cites 100M+ downloads, 12,000+ registered packages, users at 10,000+ companies and 1,500+ universities, and China as the #1 download country. These are vendor-reported and the dates are unclear. — [JuliaHub](https://juliahub.com/products/julia); [JuliaHub China post](https://info.juliahub.com/blog/downloads-of-julia-programming-language-surge-in-china)
  - Growth is concentrated in pharma (drug interactions, vaccine dosing), automotive, energy and semiconductors. — [JuliaHub](https://juliahub.com/products/julia)

### Inferences
- The two-language split is no longer accidental. It is the **industry's chosen architecture**: Python for the API, Rust/C++/CUDA underneath, with NVIDIA, Meta and Astral (uv) investing in tooling to make the seam cheaper. A new language that asks users to leave Python fights both that investment and the ecosystem's gravity.
- The seam's cost now concentrates in **(a) build and distribution of native and GPU binaries** (PEP 817: CuPy's 55 package names, torch showing up in 6.8% of uv issues) and **(b) crossing the boundary** (debugging, profiling, and type and ownership mismatches). Writing the fast code is only part of the cost.
- Julia shows that solving performance technically is not enough. Its setbacks were *latency*, *trust in correctness when composing packages* (a side effect of maximally generic multiple dispatch without interface checking), and *ecosystem pull toward PyTorch/JAX*. Julia is fixing latency in 2025–2026 (trimming, TTFX CI), roughly five years after it became the top complaint.
- The lesson for a new language: generic composition must come with *checked interfaces/contracts*, or composability turns into a liability, as in Vishnevsky's critique.

### Gaps
- I found no quantitative study (hours, dollars, or bug rates) of the engineering cost of maintaining Python/C++/CUDA bindings. PEP 817's issue-tracker numbers are the best proxy I found.
- I could not retrieve Julia Discourse replies (e.g., Chris Rackauckas's response) or 2025 Julia User Survey figures (fetch blocked).
- I found no reliable 2024–2026 data on whether Julia's ML usage (Flux/Lux) is growing or shrinking. Vendor sources only report growth.

---

## 2. Mojo / Modular: goals, open-source status (2025–2026), adoption, criticisms; Lattner's "Democratizing AI Compute" series

### Takeaway
Mojo went from a closed-compiler startup language (2023) to **1.0 on 11 Aug 2026 and a fully open-source compiler on 18 Aug 2026 (Apache-2.0 with LLVM exceptions)**. This followed **Qualcomm's ~$3.9B all-stock acquisition of Modular** (announced 24 Jun 2026, closed 29 Jul 2026). Grassroots adoption has been widely described as "cold," and the "Python superset" promise has in practice been dropped for interop. Lattner's 2025 essay series is the most thorough public diagnosis of CUDA lock-in and of why OpenCL, SYCL, AI compilers and Python eDSLs have not broken it.

### Cited Findings
**Corporate and open-source status**
- Qualcomm announced an agreement to acquire Modular on June 24, 2026, and completed it on July 29, 2026.
  - Valuation ~$3.9B, consideration mostly 18M shares of Qualcomm stock.
  - Lattner became EVP of Advanced AI Software and Platforms.
  - Mojo, MAX and Modular Cloud continue as products and brands. — [Nasdaq press release](https://www.nasdaq.com/press-release/qualcomm-completes-acquisition-modular-2026-07-29); [PR Newswire](https://www.prnewswire.com/news-releases/qualcomm-completes-acquisition-of-modular-302837286.html); [Modular blog "Qualcomm to Acquire Modular"](https://www.modular.com/blog/qualcomm-to-acquire-modular); [Qualcomm 10-Q](https://www.sec.gov/Archives/edgar/data/0000804328/000080432826000086/qcom-20260628.htm); [NAND Research analysis](https://nand-research.com/qualcomm-acquires-modular-for-its-hardware-agnostic-ai-software-layer/)
- Release and open-sourcing:
  - Mojo 1.0.0 was released August 11, 2026, with source stability.
  - On August 18, 2026 at ModCon, Modular open-sourced the entire compiler and toolchain under Apache 2.0 with LLVM exceptions.
  - The standard library has accepted outside contributions since 2024. Compiler and tooling contributions are "not ready yet," with a target of opening them by end of 2026. — [Modular blog "Mojo is now open source"](https://www.modular.com/blog/mojo-open-source); [The Register](https://www.theregister.com/ai-and-ml/2026/08/12/modulars-mojo-programming-language-hits-10-milestone/5286545); [Phoronix](https://www.phoronix.com/news/Modular-Mojo-Open-Source); [dev.to (secondary)](https://dev.to/jamilxt/mojo-vs-python-what-qualcomms-open-source-release-actually-changes-for-developers-51eg)
- ModCon 2026 also launched production Modular Cloud and extended platform support to AWS Trainium, Google TPUs and Qualcomm accelerators. — [agentic-design.ai summary (secondary)](https://agentic-design.ai/news-hub/modular-modcon-2026-open-source-open-cloud-open-silicon-b42a84); [byteiota (secondary)](https://byteiota.com/mojo-compiler-open-source-modcon-2026/)
- The `modular/modular` GitHub repo holds the Mojo compiler and stdlib, MAX kernels, the MAX inference server (OpenAI-compatible) and model pipelines. It has ~29.9k stars. MAX use is also governed by a separate "Modular Community License." — [GitHub modular/modular](https://github.com/modular/modular)

**Adoption and criticism**
- A late-2025 HN thread opened with: "Weird that there has been no significant adoption of Mojo. It has been quite some time since release…" — [HN 45138008](https://news.ycombinator.com/item?id=45138008)
- The Mojo 1.0 HN thread (409 points, per a secondary summary) split into "finally, I can try this" versus "too late, the window closed." The "too late" camp argued that the 2023 hype window is gone, the Python-superset promise is dead, and a language owned by a chip maker will always carry governance questions. — [HN 49261128](https://news.ycombinator.com/item?id=49261128); [dev.to summary (secondary)](https://dev.to/jamilxt/mojo-vs-python-what-qualcomms-open-source-release-actually-changes-for-developers-51eg)
- The realistic adoption story is swapping bottleneck modules to Mojo one at a time, not porting whole codebases. Rough edges remain around metaclasses, some decorator patterns, and libraries that assume the GIL. — [dev.to (secondary)](https://dev.to/jamilxt/mojo-vs-python-what-qualcomms-open-source-release-actually-changes-for-developers-51eg)
- Mojo gained "Python can call Mojo" interop in mid-2025. — [HN "Python can run Mojo now"](https://news.ycombinator.com/item?id=44331316); [Deep Engineering #21: Mojo–Python interop in late 2025](https://deepengineering.substack.com/p/deep-engineering-21-mojopython-interop)
- Lattner has argued publicly that ML needs a new language. — [Signals and Threads (Jane Street) "Why ML Needs a New Programming Language"](https://signalsandthreads.com/why-ml-needs-a-new-programming-language/); [Latent Space "The Shape of Compute" (2025)](https://www.latent.space/p/modular-2025)

**"Democratizing AI Compute" series (Lattner, 2025)**
- The series hub is [modular.com/democratizing-ai-compute](https://www.modular.com/democratizing-ai-compute). Among the topics: CUDA as the backbone of deep learning, why new hardware struggles to compete, and why C++ alternatives failed.
- **Part 2, "What exactly is CUDA?"**: CUDA "isn't a language – it's a whole ecosystem that has been meticulously built up over years by experts… that get bound to NV HW." — [Modular Part 2](https://www.modular.com/blog/democratizing-compute-part-2-what-exactly-is-cuda); [Lattner on X](https://x.com/clattner_llvm/status/1887257850508157348)
- **Part 5, "What about OpenCL and CUDA C++ alternatives?"**:
  - OpenCL still lacks standardized Tensor Core support, which "often results in a 5x to 10x slowdown in performance compared to using CUDA."
  - Vendors on the committee hid new hardware features until after shipping.
  - Apple abandoned OpenCL for Metal, never shipped it on iOS, and deprecated it on macOS. — [Modular Part 5](https://www.modular.com/blog/democratizing-ai-compute-part-5-what-about-cuda-c-alternatives); [Lattner on X](https://x.com/clattner_llvm/status/1897374468055687406)
- **Part 6, "What about TVM, XLA, and AI compilers?"**: covered in a third-party summary. — [zyxin.xyz summary](https://zyxin.xyz/blog/en/2025-10/democratizing-ai-compute-p6/)
- **Part 7, "What about Triton and Python eDSLs?"**:
  - A Python eDSL is "locked into a sublanguage of Python," which limits what it can express cleanly, whereas CUDA C++ can add keywords and constructs.
  - Triton "trades performance for productivity," which "prevents Triton from achieving peak efficiency," so it is "not useful for AI inference use-cases, which require maximum efficiency."
  - AI compilers face a fundamental tradeoff between abstraction and the hardware control GenAI needs.
  - The part also covers Google Pallas and NVIDIA cuTile/CUTLASS 4. — [Modular Part 7](https://www.modular.com/blog/democratizing-ai-compute-part-7-what-about-triton-and-python-edsls); [HN discussion 43507119](https://news.ycombinator.com/item?id=43507119); [Lattner on X](https://x.com/clattner_llvm/status/1905453631530566074)
  - *Date conflict:* one search summary dated Part 7 to January 8, 2026. The X post and HN item IDs point to about March 2025. Treat the date as ~March 2025 unless verified.

### Inferences
- Mojo is now a **well-capitalized, open-source, hardware-vendor-backed** entrant in exactly the "Pythonic systems language for heterogeneous compute" space. A small team starting a language here would face Mojo (Qualcomm), NVIDIA's own free Python DSLs, and PyTorch/Triton simultaneously.
- Mojo's slow grassroots adoption through 2025 supports the view that "a faster Python-like language" is not enough by itself. Developers wanted (a) true drop-in Python compatibility, (b) an open, neutral toolchain, and (c) a killer library. Mojo only reached (b) in Aug 2026, and then under a chip vendor.
- Lattner's diagnosis (hardware features hidden until launch, no standard Tensor Core abstraction, eDSLs capped by Python syntax) points to a structural gap: **no vendor-neutral language that exposes new hardware features (TMA, tensor memory, warp specialization) quickly and portably**. Mojo's pitch is that gap. Whether it delivers portability across NVIDIA/AMD/TPU/Qualcomm at peak performance is not yet independently verified.

### Gaps
- I found no independent 2025–2026 benchmarks of Mojo/MAX kernels against cuBLAS/cuDNN/FlashAttention on H100/B200 or MI300/MI355. The search budget ran out before this could be checked.
- I have no user or developer counts for Mojo, and no numbers for how many production teams use it.
- I could not read Parts 1, 3, 4, 8, 9 and later of the series directly (modular.com fetch blocked).

---

## 3. GPU programming: CUDA dominance and lock-in, the kernel-DSL explosion, and the pain kernel authors report

### Takeaway
2025–2026 brought an **explosion of tile-level kernel languages**: NVIDIA cuTile/Tile IR and CuTe DSL (CUTLASS 4), OpenAI Triton and its lower-level sibling Gluon, TileLang, ThunderKittens/HipKittens, JAX Pallas/Mosaic, Mojo, plus Rust-GPU and others. Nearly all are Python-embedded DSLs tied to one vendor or one hardware generation. Kernel authors' main complaints are:
- rewriting kernels for every new generation (Hopper → Blackwell) and every vendor (NVIDIA → AMD);
- high-level DSLs (Triton) not exposing new hardware features, which forces a drop to CuTe/CUDA;
- immature non-NVIDIA software;
- fragmented tooling with weak debugging.

### Cited Findings
**CUDA dominance**
- NVIDIA said at GTC 2025 that more than one million developers build AI applications with CUDA-X. — [Counterpoint Research](https://counterpointresearch.com/en/insights/post-insight-nvidia-gtc-2025-gpu-tokens-collaborations)
- SemiAnalysis, Dec 2024 (5-month study): H100/H200 beat MI300X by **>2.5× in achieved training TFLOP/s**, despite MI300X's better paper specs and TCO, because of software. FlexAttention was not working on AMD at their deadline, while it had worked on NVIDIA since Aug 2024. — [SemiAnalysis "CUDA Moat Still Alive"](https://newsletter.semianalysis.com/p/mi300x-vs-h100-vs-h200-benchmark-part-1-training); [Dylan Patel on X](https://x.com/dylan522p/status/1870960578338173007)
- By Oct 2025, SemiAnalysis said ROCm-specific bugs were "orders of magnitude lower" than in 2024. — [SemiAnalysis on X](https://x.com/SemiAnalysis_/status/1977571931504153076)
- AMD shipped ROCm 10 with "ROCm.AI," an AI-assisted developer experience, in Sept 2026. It claims 1M+ Hugging Face models work out of the box. — [AMD newsroom](https://newsroom.amd.com/news/rocm-10-software-ai-native-developer-experiences/); [AMD blog](https://www.amd.com/en/blogs/2026/rocm-ai-the-ai-native-developer-experience-for-building.html)

**NVIDIA's own Python kernel languages (2025)**
- **cuTile Python / CUDA Tile**:
  - Shipped in CUDA 13.1 (Dec 2025). It is a Python DSL targeting **Tile IR, a new virtual ISA/MLIR dialect** for NVIDIA GPUs.
  - Users program tiles rather than threads, and the compiler handles block-level parallelism, async memory movement, tensor cores and TMA.
  - Initially Blackwell-only. CUDA 13.2 added Ampere and Ada. The repo says **Hopper support is "planned."** It requires driver r580+. — [PyPI cuda-tile](https://pypi.org/project/cuda-tile/); [NVIDIA/cutile-python GitHub](https://github.com/NVIDIA/cutile-python); [Hackster](https://www.hackster.io/news/nvidia-makes-gpu-acceleration-easier-more-portable-with-cuda-tiles-cutile-python-b808c1ad7e4d); [NVIDIA blog tag cuda-tile](https://developer.nvidia.com/blog/tag/cuda-tile/); [Spheron (secondary)](https://www.spheron.network/blog/cuda-13-tile-programming-gpu-cloud/)
- **CUTLASS 4 / CuTe DSL** (May 2025):
  - Python-native device-kernel authoring with the same abstractions as CuTe C++.
  - Claims "performance on par with C++ kernels and **100x+ faster compile times**," plus no C++ template metaprogramming.
  - Public beta, expected to leave beta by summer 2026. — [NVIDIA HPC Dev on X](https://x.com/NVIDIAHPCDev/status/1922747315762200990); [CUTLASS DSL docs](https://docs.nvidia.com/cutlass/latest/media/docs/pythonDSL/overview.html); [NVIDIA dev blog](https://developer.nvidia.com/blog/achieve-cutlass-c-performance-with-python-apis-using-cute-dsl/); [PyPI nvidia-cutlass-dsl](https://pypi.org/project/nvidia-cutlass-dsl/)

**Triton and its limits**
- FlashAttention-4 on Blackwell (SM100) moved to CuTe DSL because Blackwell's TMA and tensor-memory hardware need tile-level control that Triton's abstractions did not fully expose. — [Spheron Triton guide (secondary)](https://www.spheron.network/blog/openai-triton-kernel-gpu-cloud-2026/)
- **Gluon** is a Triton-family, lower-level DSL that exposes compiler internals and explicit controls to reach higher performance ceilings. It is used for performance-critical attention and MoE kernels on AMD. — [Lei.Chat "Gluon: Explicit Performance"](https://www.lei.chat/posts/gluon-explicit-performance/); [ROCm blog Gluon GEMM tutorial](https://rocm.blogs.amd.com/software-tools-optimization/gluon-gemm-tutorial/README.html)
- vLLM (Mar 2026): "maintaining hundreds of kernels across multiple GPU platforms like NVIDIA Hopper and Blackwell, AMD MI300, Intel… quickly becomes impractical." Triton still needs manual meta-parameter tuning, and new generations change architectural primitives. — [vLLM blog, Triton attention backend](https://vllm.ai/blog/2026-03-04-vllm-triton-backend-deep-dive); [arXiv 2511.11581 "Anatomy of a Triton Attention Kernel"](https://arxiv.org/pdf/2511.11581); [PyTorch blog TokenSpeed-Kernel "portable APIs… multi-silicon"](https://pytorch.org/blog/lightseek-tokenspeed-kernel/)

**Academic and startup DSLs**
- **ThunderKittens** (Stanford Hazy Research):
  - A C++/CUDA-embedded tile-primitive library with "simplicity, extensibility, speed" as its principles.
  - Claims a GEMM of ~**855 TFLOPs on H100 (86% of peak) in <100 lines**.
  - Supports Hopper and Blackwell. Ampere is no longer actively supported, and Vera Rubin support was added Sept 2026. — [ThunderKittens GitHub](https://github.com/HazyResearch/ThunderKittens)
- **HipKittens** (AMD port, MLSys 2026):
  - Finds that some interfaces transfer from NVIDIA, but "memory access patterns, scheduling compute and memory, and ordering thread blocks within the chiplet architecture differ." The project is "built from the hardware up."
  - Benchmarked against AITER, hipBLASLt and PyTorch. — [HipKittens GitHub](https://github.com/HazyResearch/HipKittens); [arXiv 2511.08083](https://arxiv.org/html/2511.08083v1)
- **TileLang** (arXiv 2504.17577):
  - A Python-like tile DSL on top of TVM. Unlike Triton, it lets users declare buffers explicitly at each memory level, with layout inference.
  - Backends: NVIDIA SM70–SM120 and AMD CDNA/RDNA as primary targets; Apple Metal also supported; experimental CPU, CuTe DSL and WebGPU.
  - External ports exist for Huawei Ascend, MetaX, Moore Threads, Hygon and Sunrise.
  - **DeepSeek uses it** (MLA, V3.2, V4 examples). — [TileLang paper](https://arxiv.org/pdf/2504.17577); [tile-ai/tilelang GitHub](https://github.com/tile-ai/tilelang)
- **JAX Pallas / Mosaic**:
  - JAX's kernel extension for TPU and GPU, with Mosaic as an MLIR backend and escape hatch from XLA.
  - Kernel authors must manage VMEM/SMEM/HBM, software pipelining, block-shape constraints and grid order. — [Pallas docs](https://docs.jax.dev/en/latest/pallas/index.html); [Mosaic GPU reference](https://docs.jax.dev/en/latest/pallas/gpu/reference.html)
  - Pallas appears "orders of magnitude less frequently" than CUDA or Triton in LLM training data, so models "routinely hallucinate Pallas APIs." — [JAXBench, arXiv 2607.20466](https://arxiv.org/pdf/2607.20466); a related "low-resource GPU DSL" problem is studied in [DSL-Monkeys (OpenReview)](https://openreview.net/forum?id=2yS4j1C3zi)
- **Bend / HVM2** (Higher Order Company):
  - Per Taelin (late 2025), a full RTX 4090 was needed to beat 1-core OCaml/JS because of interpretation overhead. — [Taelin on X](https://x.com/VictorTaelin/status/1985423273304473788); [HN 2024 launch thread](https://news.ycombinator.com/item?id=40390287)
  - The current README has pivoted to "a fast language that blocks AI mistakes via proof." It warns "BEND IS YOUNG. EXPECT BUGS," says the compiler is "99% AI-written and has not been fully audited," and supports "one GPU per program… no multi-machine execution yet." — [HigherOrderCO/Bend GitHub](https://github.com/HigherOrderCO/Bend)
- **Rust**:
  - A July 2025 demo ran one Rust codebase on CUDA, SPIR-V/Vulkan, Metal, DX12, WebGPU and CPU with no shader languages. — [Rust GPU blog](https://rust-gpu.github.io/blog/2025/07/25/rust-on-every-gpu/)
  - Rust-CUDA was rebooted in Jan 2025 after 3+ dormant years. Its README says: "Expect bugs, safety issues, and things that don't work." — [Rust CUDA reboot](https://rust-gpu.github.io/blog/2025/01/27/rust-cuda-reboot/); [Rust-GPU/rust-cuda GitHub](https://github.com/Rust-GPU/rust-cuda)

**Other ecosystems**
- **Apple MLX**: Apple's array framework, with Python, Swift, C++ and C APIs, a custom Metal kernel API (`mx.fast`), and Metal 4 TensorOps for the M5 GPU neural accelerators. — [Apple ML Research](https://machinelearning.apple.com/research/exploring-llms-mlx-m5); [WWDC25 session](https://developer.apple.com/videos/play/wwdc2025/315/)
- **SYCL/oneAPI → UXL Foundation**: founding steering members are Arm, Fujitsu, Google, Imagination, Intel, Qualcomm and Samsung. — [oneAPI UXL announcement](https://oneapi.io/blog/announcing-the-unified-acceleration-uxl-foundation/)
- **TVM**: NVIDIA acquired OctoAI, the TVM commercializer, in Sept 2024 (reported $165M–$250M) and shut its service on Oct 31, 2024. TVM's vision continues partly in MLC-LLM. — [GeekWire](https://www.geekwire.com/2024/chip-giant-nvidia-acquires-octoai-a-seattle-startup-that-helps-companies-run-ai-models/); [BigDATAwire](https://www.hpcwire.com/bigdatawire/2024/09/30/octoai-snapped-up-by-nvidia/)

### Inferences
- **The central pain is "performance portability debt"**:
  - Each new NVIDIA generation (Hopper wgmma/TMA → Blackwell tcgen05/TMEM) and each vendor (AMD's chiplet memory model) forces kernel rewrites. FA4 left Triton for CuTe DSL, and HipKittens had to rethink scheduling for AMD.
  - Even NVIDIA's own cuTile skipped Hopper at launch.
  - The industry responds with *more DSLs*, which splits kernel code further.
- **The eDSL ceiling is widely acknowledged.** Triton-style DSLs are productive but cap performance. Lower-level DSLs (Gluon, CuTe DSL, TK) reach peak by exposing hardware, and so lose portability. No current language gives a *single source* that is both explicitly controllable and retargetable.
- **Vendor capture**: the most capable new DSLs belong to NVIDIA (cuTile, CuTe DSL), Qualcomm (Mojo), Google (Pallas) or OpenAI (Triton). Neutral, community-governed options are academic (TileLang, TK/HK, Exo) or immature (Rust-GPU/CUDA, Bend).
- **An LLM cold-start barrier now shapes adoption**: models write CUDA and Triton well but hallucinate low-resource DSLs such as Pallas. Any new kernel language must plan for AI-assisted authoring, e.g., a small, regular surface, strong static checking, and machine-checkable specs.

### Gaps
- I found no systematic survey of kernel-author pain (debugging tools, compile times, race detection) with quantitative data. The evidence is anecdotal or comes from vendor and research blogs.
- I did not verify independent benchmark numbers for cuTile against Triton or CuTe, for TileLang against FlashAttention-3, or for Pallas on GPU.
- I did not cover WebGPU/WGSL's 2025 cross-browser status or Halide's current production use; the search budget ran out.

---

## 4. Systems and architecture research (ASPLOS, OSDI, SOSP, MLSys, CGO, PLDI 2023–2026)

### Takeaway
Research is converging on four themes:
1. **user-schedulable languages** that separate the algorithm from its schedule (Halide lineage → Exo 2, Exo-GPU);
2. **tile-level DSLs and compilers** (TileLang, ThunderKittens/HipKittens);
3. **automatic fusion and megakernels / superoptimization** (Mirage, PyTorch 2);
4. **safety for GPU code** (Descend, "Fearless Concurrency on the GPU," Rust GPU offload).

Energy-efficiency research has shifted from "which language is greenest" toward "implementation, core count and memory behavior dominate."

### Cited Findings
- **PyTorch 2 (ASPLOS '24)**: TorchDynamo plus TorchInductor, 2.27× inference and 1.41× training geomean speedup on A100 over 180+ models, beating six other compilers. — [ACM DL](https://dl.acm.org/doi/10.1145/3620665.3640366)
- **Exo 2: Growing a Scheduling Language (ASPLOS '25)**:
  - Users define new scheduling operations outside the compiler by composing trusted primitives.
  - MIT says this matches state-of-the-art libraries with "a few hundred lines of code, instead of tens or hundreds of thousands."
  - The category is "user-schedulable languages" (term attributed to Ragan-Kelley). — [ACM DL](https://dl.acm.org/doi/10.1145/3669940.3707218); [arXiv 2411.07211](https://arxiv.org/pdf/2411.07211); [MIT News, Mar 2025](https://news.mit.edu/2025/high-performance-computing-with-much-less-code-0313); [exo-lang.dev](https://exo-lang.dev/)
  - Follow-on: "Exo-GPU: Safe, Imperative, User-schedulable Programming for Tensor Cores." — [arXiv 2609.16389 (Sept 2026)](https://arxiv.org/pdf/2609.16389)
- **Mirage: A Multi-Level Superoptimizer for Tensor Programs (OSDI '25)**: automatically finds fused kernels. Its Mirage Persistent Kernel (MPK, June 2025) compiles multi-GPU LLM inference into a *single megakernel* and reduces latency 1.2×–6.7×. — [mirage-project GitHub](https://github.com/mirage-project/mirage)
- **FlashInfer**: an attention/GEMM/MoE kernel library and generator used by vLLM, SGLang, TensorRT-LLM, TGI, MLC-LLM and others. It mixes CUDA, CuTe DSL (for Blackwell) and JIT. — [flashinfer GitHub](https://github.com/flashinfer-ai/flashinfer); paper [arXiv 2501.01005](https://arxiv.org/abs/2501.01005)
- **HipKittens (MLSys 2026)**: AMD tile primitives. — [HipKittens GitHub](https://github.com/HazyResearch/HipKittens)
- **TileLang (2025)**: [arXiv 2504.17577](https://arxiv.org/pdf/2504.17577)
- **KernelBench (ICML '25)**:
  - Measures whether LLMs can write correct and faster kernels, using the `fast_p` metric (fraction of tasks that are correct *and* faster than PyTorch by a factor p).
  - The team explicitly "does not review, validate, or endorse individual kernels or reported results." — [KernelBench GitHub](https://github.com/ScalingIntelligence/KernelBench)
- **Descend (PLDI '24, PACMPL)**:
  - A Rust-style ownership and borrow system for GPU code that statically rejects data races and bad synchronization.
  - It schedules computation hierarchically over the grid, blocks, warps and threads.
  - The paper reports performance matching CUDA. — [ACM DL](https://dl.acm.org/doi/10.1145/3656411); [arXiv 2305.03448](https://arxiv.org/abs/2305.03448)
  - Related 2026 preprints: "Fearless Concurrency on the GPU" ([arXiv 2606.15991](https://arxiv.org/pdf/2606.15991)); "GPU Offload in Rust: Portable, Safe, and Fast" ([arXiv 2608.13759](https://arxiv.org/html/2608.13759v1)); "Modular GPU Programming with Typed Perspectives" ([arXiv 2511.11939](https://arxiv.org/pdf/2511.11939)).
- **Accelerator retargeting**:
  - "Pushing Tensor Accelerators beyond MatMul in a User-Schedulable Language" uses Halide plus equality saturation for tensor instruction selection on CPU- and GPU-attached accelerators. — [arXiv 2512.02371](https://arxiv.org/pdf/2512.02371)
  - "Automatically Generating ML Compiler Backends from Tensor Accelerator ISA Descriptions." — [arXiv 2510.09932](https://arxiv.org/pdf/2510.09932)
  - "Autocomp" (LLM-driven optimizer for tensor accelerators). — [arXiv 2505.18574](https://arxiv.org/pdf/2505.18574)
  - "Dato: task-based programming for dataflow accelerators." — [arXiv 2509.06794](https://arxiv.org/pdf/2509.06794)
  - **[background]** TensorIR (ASPLOS '23). — [ACM DL](https://dl.acm.org/doi/10.1145/3575693.3576933); curated list of tensor compilers: [awesome-tensor-compilers](https://github.com/merrymercy/awesome-tensor-compilers)
- **Energy efficiency of languages**:
  - **[background]** Pereira et al.'s "Energy Efficiency across Programming Languages" (SLE 2017; SCP 2021) found Python ~75× C's energy. — [ScienceDirect](https://www.sciencedirect.com/science/article/pii/S0167642321000022); [green-coding.io 2024 revisit (Python 3.12, PyPy, Mojo)](https://www.green-coding.io/case-studies/energy-efficiency-python/)
  - "It's Not Easy Being Green" (2024) critiques that work. It finds that language-energy associations were misinterpreted, and that implementation, application code, number of active cores and memory activity drive energy use more than the language itself. — [arXiv 2410.05460](https://arxiv.org/html/2410.05460v1); [IEEE Xplore](https://ieeexplore.ieee.org/document/11334459/); [HN](https://news.ycombinator.com/item?id=41801018)
  - Idle/low-batch GPUs still draw a large share of peak power: at batch 1, under 1.3% MFU but 49–64% of batch-128 power. — [arXiv 2608.03880](https://arxiv.org/html/2608.03880v1)

### Inferences
- Research has largely *accepted* that fully automatic compilers cannot reliably reach peak on new accelerators. The frontier is **programmer-controlled but compiler-checked** transformation: schedules (Exo 2), explicit tiles (TileLang/TK), and verified rewrites (Mirage).
- A new language that makes *schedules/tiling first-class and type-checked*, with *safety guarantees in the style of Descend*, would sit where several active research threads meet but no production language currently is.
- The energy argument for a new language is weak if framed as "language X is greener." It is strong if framed as **raising accelerator utilization**, because idle or low-MFU GPUs still draw most of their power.

### Gaps
- I did not retrieve SOSP 2025, CGO 2025/2026 or PLDI 2025/2026 program lists (usenix.org and dl.acm.org fetches blocked, search budget exhausted).
- I did not verify the FlashInfer MLSys 2025 award or KernelBench headline numbers for frontier models.
- The 2026 arXiv preprints above were identified by title and abstract snippet only.

---

## 5. Python performance efforts (CPython JIT, free-threading, PyPy, Codon, SPy, Cinder): do they close the gap?

### Takeaway
No. CPython's own efforts give **single-digit to ~30% gains** (JIT) and **multicore scaling at a ~1–10% single-thread cost** (free-threading), compared with the 10–100× gap to native code. Microsoft's funding cut in May 2025 slowed the Faster CPython effort. Compiled Python dialects (Codon, SPy, Mojo) reach native speed only by **dropping dynamic features**, which reintroduces a second (sub)language.

### Cited Findings
**CPython JIT**
- PEP 744 (Apr 2024, Python 3.13) specifies a copy-and-patch JIT generated from the interpreter's DSL (~900 lines of build-time Python, ~500 lines of runtime C).
  - At the time it "isn't yet a clear win when always enabled."
  - The bar for becoming non-experimental is a ~5% improvement on at least one popular platform. — [PEP 744](https://github.com/python/peps/blob/main/peps/pep-0744.rst)
- CPython 3.14 JIT results (secondary sources):
  - 10–30% speedups on pure-Python loops (Mandelbrot, Levenshtein) and ~20% on richards.
  - ~10% slowdown on nbody.
  - 0% on I/O-bound or C-extension-heavy code.
  - With Clang 20 builds, the interpreter often beats the JIT. — [Towards Data Science](https://towardsdatascience.com/python-3-14-and-its-new-jit-compiler/); [InfoWorld CPython vs PyPy](https://www.infoworld.com/article/4117428/which-python-runtime-does-jit-better-cpython-or-pypy.html)
- July 2025: "Despite 30 months work, core developer says Python's JIT compiler is often slower than the interpreter." — [DevClass](https://devclass.com/2025/07/09/despite-30-months-work-core-developer-says-pythons-jit-compiler-is-often-slower-than-the-interpreter/); [Ken Jin, "Reflections on 2 years of CPython's JIT"](https://fidget-spinner.github.io/posts/jit-reflections.html); [LWN follow-up](https://lwn.net/Articles/1029307/)
- Microsoft cancelled support for the Faster CPython project in **May 2025** and laid off most of the team, including core developers Eric Snow, Irit Katriel and Mark Shannon. The JIT continued as a community project, and on 17 Mar 2026 Ken Jin reported the 3.15 JIT "back on track." — [HN](https://news.ycombinator.com/item?id=45603580); [discuss.python.org "Community Stewardship of Faster CPython"](https://discuss.python.org/t/community-stewardship-of-faster-cpython/92153); [Python Insider, Mar 2026](https://blog.python.org/2026/03/jit-on-track/); [Real Python 3.15 JIT preview](https://realpython.com/python315-jit-compiler/)

**Free-threading (PEP 703/779)**
- PEP 779 (Final, resolved June 16, 2025) made free-threaded Python officially supported in 3.14 as "Phase II."
  - Hard targets: ≤15% single-thread slowdown and ≤20% memory overhead.
  - Measured at the time: about 10% slower on Linux/Windows, ~3% on macOS, and 15–20% more memory. — [PEP 779](https://github.com/python/peps/blob/main/peps/pep-0779.rst)
- Later 3.14 measurements put the pyperformance overhead at ~1% (macOS aarch64) to ~8% (x86-64 Linux). Practitioners report ecosystem breakage for C extensions that assume the GIL. — [danilchenko.dev (secondary)](https://www.danilchenko.dev/posts/python-314-free-threading/); [dev.to (secondary)](https://dev.to/dmaxdev/python-314-free-threading-real-benchmarks-real-breakage-real-code-3m5); [Python docs: free-threading HOWTO](https://docs.python.org/3/howto/free-threading-python.html)

**Compiled dialects**
- **Codon** (MIT/Exaloop) **[background 2023, still current]**:
  - Typical 10–100× single-thread speedups over Python, sometimes reaching C/C++ speed.
  - Not a drop-in replacement: no runtime polymorphism, reflection, dynamic method-table changes, metaclasses or class decorators, and it has stdlib gaps.
  - Offers a `@codon.jit` decorator and a GPU/parallel backend. — [USENIX ;login:](https://www.usenix.org/publications/loginonline/codon-python-compiler); [MIT News 2023](https://news.mit.edu/2023/codon-python-based-compiler-achieve-orders-magnitude-speedups-0314); [The New Stack "faster… (for some workloads)"](https://thenewstack.io/codon-is-a-faster-python-compiler-for-some-workloads/)
- **SPy** (Antonio Cuni, Anaconda; talks at PyCon US 2024 and PyCon Italia 2025):
  - A statically typed Python variant built on the premise that "modern Python" is already a mostly statically typed subset. It aims at "C-like speed" (10–100× CPython).
  - Has an interpreter for development and compiles to C, WASI and Emscripten.
  - Status is alpha, with "very scarce" docs. — [Cuni blog "Inside SPy part 1" (Oct 2025)](https://antocuni.eu/2025/10/29/inside-spy-part-1-motivations-and-goals/); [spylang/spy GitHub](https://github.com/spylang/spy); [PyCon Italia 2025](https://2025.pycon.it/en/event/spy-static-python-lang-fast-as-c-pythonic-as-python)
- Mojo interop limits: metaclasses, decorator patterns and GIL assumptions. — [dev.to (secondary)](https://dev.to/jamilxt/mojo-vs-python-what-qualcomms-open-source-release-actually-changes-for-developers-51eg)

### Inferences
- The CPython work (JIT plus free-threading) matters for general Python, but it structurally **cannot remove the need for native kernels** in ML or scientific computing. Heavy work already runs in C, CUDA or Rust, where the JIT gives "0%."
- Every approach that reaches native speed does so by defining a **static subset** (Codon, SPy, Mojo, Numba, Triton, cuTile). The industry is effectively re-deriving "typed Python" many times over with incompatible semantics. That duplication is itself a pain point and a possible opening: one shared, well-specified static-Python core that many backends (CPU, GPU tile, WASM) accept.
- Funding fragility matters. Microsoft's May 2025 cut shows that even core-Python performance work depends on one corporate sponsor, a warning for any small-team language effort.

### Gaps
- I have no exact 3.15 JIT benchmark numbers (blog.python.org fetch blocked).
- I found no current (2025–2026) information on PyPy's status or funding, or on Cinder/Static Python at Meta; the search budget ran out.
- I found no reliable data on how many PyPI packages have free-threaded wheels.

---

## 6. How big is the affected population, and how much money is at stake?

### Takeaway
The money at stake is very large: hyperscaler 2026 capex is estimated at **~$660–800B**, most of it AI. Utilization of that hardware is mediocre: typical production LLM training reaches **~35–55% MFU**, and decode inference only **~8–12%**. So a few percentage points of efficiency are worth billions. But the population that writes performance-critical kernels is **small** (likely thousands to low tens of thousands, not reliably measured). It sits on top of a much larger population (millions) of Python ML and scientific developers who consume those kernels.

### Cited Findings
- Hyperscaler capex (secondary aggregators; they differ):
  - The Big-5 (Amazon, Alphabet, Meta, Microsoft, Oracle) planned roughly **$660–690B** of 2026 capex. — [Futurum](https://futurumgroup.com/insights/ai-capex-2026-the-690b-infrastructure-sprint/)
  - After Q1 2026 earnings (Apr 29, 2026), one analysis put combined 2026 capex at **~$775–800B** (+~64% YoY), with ~75% (~$545B) AI-specific. — [AL Capital Advisory](https://alcapitaladvisory.com/research/intelligence/ai-infrastructure.html)
  - Other estimates: $600–630B. — [Introl](https://introl.com/blog/hyperscaler-capex-600b-2026-ai-infrastructure-debt-january-2026); [Data Center Richness](https://datacenterrichness.substack.com/p/hyperscalers-plan-630-billion-in)
- Utilization and MFU (practitioner and secondary explainers):
  - Production LLM training typically reaches 35–45% MFU. Well-tuned dense pretraining on H100 with FlashAttention 2/3 reaches 40–55%; MoE 25–40%; inference decode at batch 32 only 8–12% (bandwidth-bound).
  - NVIDIA-reported runs: Llama-3.1-70B on 16× HGX B200 at 50–53% MFU; Llama-405B on GB300 NVL72 at 52–56%. — [ZeroEntropy MFU explainer](https://zeroentropy.dev/concepts/mfu/); [Glenn Lockwood MFU notes](https://www.glennklockwood.com/garden/mfu); [TechnoLynx](https://www.technolynx.com/post/model-flops-utilization-ai-training); [Lambda MFU white paper](https://lambda.ai/hubfs/4.%20Resources/White%20Papers/Lambda%20MFU.pdf)
  - At batch 1, GPUs run at <1.3% MFU yet draw 49–64% of batch-128 power. — [arXiv 2608.03880](https://arxiv.org/html/2608.03880v1)
- Developer populations:
  - NVIDIA: 1M+ developers building AI apps with CUDA-X (GTC 2025). — [Counterpoint](https://counterpointresearch.com/en/insights/post-insight-nvidia-gtc-2025-gpu-tokens-collaborations)
  - Python: 51% of 30k+ surveyed Python developers do data exploration and processing. — [JetBrains State of Python 2025](https://blog.jetbrains.com/pycharm/2025/08/the-state-of-python-2025/)
  - Julia: "more than 1 million users." — [MIT News 2026](https://news.mit.edu/2026/how-mit-research-project-became-global-programming-language-0831)
  - Mojo: `modular/modular` has ~29.9k GitHub stars. — [GitHub](https://github.com/modular/modular)
- Strategic value of the software layer: Qualcomm paid ~$3.9B for Modular (a language, compiler and inference stack). NVIDIA bought OctoAI (TVM) for a reported $165–250M. — [Nasdaq](https://www.nasdaq.com/press-release/qualcomm-completes-acquisition-modular-2026-07-29); [GeekWire](https://www.geekwire.com/2024/chip-giant-nvidia-acquires-octoai-a-seattle-startup-that-helps-companies-run-ai-models/)
- The SemiAnalysis Dec 2024 finding that MI300X delivered <40% of H100/H200's achieved training throughput *because of software* puts a number on the value of the software layer: hardware with better specs and TCO lost. — [SemiAnalysis](https://newsletter.semianalysis.com/p/mi300x-vs-h100-vs-h200-benchmark-part-1-training)

### Inferences
- **Pain is concentrated.** Kernel and compiler engineers (few) create the efficiency that millions of Python users consume. A new language aimed at kernel authors reaches few people but can capture a lot of value. A language aimed at all Python ML users reaches many people but competes with PyTorch and Python itself.
- **Hardware diversification increases the pain**: AMD MI3xx/MI4xx, TPU, Trainium, Qualcomm, Apple, and Chinese accelerators (TileLang ports). Hardware buyers have a strong incentive to fund portability, which explains the Qualcomm-Modular deal. It also means any winner in this space is likely to be vendor-funded or acquired.
- Even at ~40% MFU, the gap to peak on ~$545B/year of AI-specific capex is huge. Only part of that gap is a *language* problem; much of it is networking, memory bandwidth and parallelism strategy.

### Gaps
- I found **no reliable count of GPU kernel authors**, ML engineers, or scientific/HPC programmers worldwide. NVIDIA's CUDA-X figure is a vendor number that counts users, not kernel authors.
- A secondary source claimed "$180B of GPU/accelerator spend ≈ 6M GPUs at ~$30K"; I could not tie it to a specific primary source.
- I found no study that isolates how much of the MFU gap is caused by kernel or language quality versus systems-level factors.

---

## 7. Where is the remaining gap that no current language fills? Can a small team win?

### Takeaway
The unfilled gap is a **vendor-neutral, single-source language for accelerator kernels and the host code around them**. It would need four properties together:
1. explicit, *type-checked* control over tiles, memory spaces and schedules, reaching peak like CuTe/TK;
2. *retargetability* across NVIDIA generations and vendors, like Triton/TileLang aim for;
3. *static safety* for races and synchronization, like Descend;
4. *first-class Python interop and packaging*.

Every existing option gives up at least one of these. A small team is unlikely to win head-on against Mojo/Qualcomm, NVIDIA's free DSLs, and Triton/PyTorch. It *could* win a narrower wedge, such as verified or safe kernels, schedule-first portability for non-NVIDIA accelerators, or a language designed for AI-assisted kernel authoring.

### Cited Findings (evidence for specific gaps)
- **Portability versus peak**:
  - Triton "trades performance for productivity." — [Lattner Part 7](https://www.modular.com/blog/democratizing-ai-compute-part-7-what-about-triton-and-python-edsls)
  - FA4 left Triton for CuTe DSL on Blackwell. — [Spheron (secondary)](https://www.spheron.network/blog/openai-triton-kernel-gpu-cloud-2026/)
  - Gluon exists to expose lower-level control. — [Lei.Chat](https://www.lei.chat/posts/gluon-explicit-performance/)
  - vLLM calls multi-platform kernel maintenance "impractical." — [vLLM blog](https://vllm.ai/blog/2026-03-04-vllm-triton-backend-deep-dive)
  - AMD needs different scheduling and memory decisions. — [HipKittens](https://github.com/HazyResearch/HipKittens)
- **Vendor lock-in of the new DSLs**:
  - cuTile/Tile IR is NVIDIA-only, and Hopper support was still "planned." — [cutile-python](https://github.com/NVIDIA/cutile-python)
  - CuTe DSL is NVIDIA-only. — [CUTLASS docs](https://docs.nvidia.com/cutlass/latest/media/docs/pythonDSL/overview.html)
  - Mojo is now owned by Qualcomm, with governance concerns raised by practitioners. — [dev.to (secondary)](https://dev.to/jamilxt/mojo-vs-python-what-qualcomms-open-source-release-actually-changes-for-developers-51eg)
  - Standards bodies failed because vendors hid features until launch. — [Lattner Part 5](https://www.modular.com/blog/democratizing-ai-compute-part-5-what-about-cuda-c-alternatives)
- **Safety**:
  - CUDA and OpenCL inherit C/C++'s unsafe raw-pointer memory access, which makes data races and deadlocks easy to write and hard to detect. Descend shows safety at CUDA performance, but only as a research prototype. — [Descend](https://dl.acm.org/doi/10.1145/3656411)
  - Rust-CUDA warns of "safety issues." — [rust-cuda](https://github.com/Rust-GPU/rust-cuda)
- **eDSL expressiveness ceiling**: Python eDSLs are "locked into a sublanguage of Python." — [Lattner Part 7](https://www.modular.com/blog/democratizing-ai-compute-part-7-what-about-triton-and-python-edsls)
- **Proliferating typed-Python subsets**: Codon ([USENIX](https://www.usenix.org/publications/loginonline/codon-python-compiler)), SPy ([GitHub](https://github.com/spylang/spy)), Numba-CUDA "restricted subset of Python" ([cuda-python](https://github.com/NVIDIA/cuda-python)), cuTile, Triton, Pallas, Mojo.
- **Packaging and distribution of native/GPU code**: [PEP 817](https://github.com/python/peps/blob/main/peps/pep-0817.rst)
- **AI-authored kernels**:
  - LLM kernel-generation benchmarks exist (KernelBench `fast_p`, ICML '25), but the maintainers do not validate reported results. — [KernelBench](https://github.com/ScalingIntelligence/KernelBench)
  - Low-resource DSLs are hallucinated by LLMs. — [JAXBench](https://arxiv.org/pdf/2607.20466); [DSL-Monkeys](https://openreview.net/forum?id=2yS4j1C3zi)
  - Bend's compiler is "99% AI-written and has not been fully audited." — [Bend](https://github.com/HigherOrderCO/Bend)
- **Schedules as a language feature**: Exo 2 shows schedule libraries replacing "tens or hundreds of thousands" of lines. — [MIT News](https://news.mit.edu/2025/high-performance-computing-with-much-less-code-0313)

### Inferences
**Assessing the size of the pain**
- **Very high value per user, small user base, crowded field.** Pain severity is high: generation-by-generation rewrites, vendor lock-in, and software (not silicon) deciding hardware competitiveness. The money is enormous (hundreds of billions per year in capex, roughly 40% MFU). Direct users (kernel and compiler engineers) are few.
- **Incumbents and fast followers are strong:**
  - NVIDIA ships free Python DSLs (cuTile, CuTe DSL) with first-day access to new hardware features.
  - Mojo is open source with Qualcomm's balance sheet.
  - Triton, Gluon and PyTorch Inductor have ecosystem lock-in.
  - Academic DSLs (TileLang, TK/HK, Exo) iterate quickly and are free.
  - A small team competing on "yet another tile DSL" would be the 8th-plus entrant.

**Where a small team could realistically win (wedges)**
1. **Safety and verification for GPU kernels.** No production language offers Rust-like race freedom plus tile-level performance. Research (Descend, Exo-GPU "safe," "Fearless Concurrency on the GPU") shows it is feasible. It matters more as LLMs write kernels (KernelBench shows the need for validation). This fits a small team because it is deep PL work, not ecosystem breadth.
2. **Schedule-first portability for the non-NVIDIA long tail** (AMD, Qualcomm, TPU, Chinese NPUs, Apple). Vendors there *want* a neutral language and would fund it; TileLang's many ports hint at demand. The risk is acquisition or capture, as with Modular and OctoAI.
3. **One shared static-Python core and ABI** that Codon, SPy, Numba, Triton and cuTile-style subsets could all target, together with build and packaging solved end to end, addressing the PEP 817 pain. This is a standards and tooling play more than a new language.
4. **Designing for AI-assisted authoring**: a small, regular, strongly checked surface with machine-checkable specifications and performance contracts, so LLM-generated kernels can be verified. This avoids the "low-resource DSL" handicap by making correctness checkable rather than depending on training-data volume.

**Where a small team would likely lose**
- A general "faster Python" language (Mojo's original pitch): Mojo, Codon, SPy and CPython itself are already there, and adoption has been slow even with $100Ms of funding.
- Replacing CUDA wholesale: Lattner's own series explains why OpenCL and SYCL failed despite industry consortia.

**Julia's lesson for any new entrant:** the aim should be trust (checked interfaces, no silent composability bugs), low latency from day one, and seamless Python and packaging interop. Raw speed alone was not enough.

### Gaps
- I found no market sizing for "kernel-language tooling" as a product category. Deal values (Modular ~$3.9B, OctoAI ~$165–250M) are the only price signals.
- I found no practitioner survey quantifying which kernel pain (portability, debugging, compile time, safety, packaging) matters most. The ranking above is inferred from blogs and papers.
- I did not verify how well Mojo actually performs across NVIDIA, AMD, TPU and Qualcomm; this is critical for judging whether the "vendor-neutral kernel language" gap is already closing.
