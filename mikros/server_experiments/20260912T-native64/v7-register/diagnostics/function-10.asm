000000000065d3d0 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176>:
  65d3d0:	push   rbp
  65d3d1:	push   r15
  65d3d3:	push   r14
  65d3d5:	push   r13
  65d3d7:	push   r12
  65d3d9:	push   rbx
  65d3da:	sub    rsp,0x1000
  65d3e1:	mov    QWORD PTR [rsp],0x0
  65d3e9:	sub    rsp,0x1000
  65d3f0:	mov    QWORD PTR [rsp],0x0
  65d3f8:	sub    rsp,0xe38
  65d3ff:	mov    rax,QWORD PTR [rdi]
  65d402:	mov    rbx,QWORD PTR [rax]
  65d405:	mov    QWORD PTR [rax],0x0
  65d40c:	test   rbx,rbx
  65d40f:	je     6605fd <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x322d>
  65d415:	mov    edi,0x10000
  65d41a:	mov    esi,0x1
  65d41f:	call   QWORD PTR [rip+0x9ae13]        # 6f8238 <calloc@GLIBC_2.2.5>
  65d425:	test   rax,rax
  65d428:	je     66060a <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x323a>
  65d42e:	vpxor  xmm0,xmm0,xmm0
  65d432:	vmovdqu YMMWORD PTR [rsp+0x570],ymm0
  65d43b:	vmovdqu YMMWORD PTR [rsp+0x550],ymm0
  65d444:	vmovdqu YMMWORD PTR [rsp+0x530],ymm0
  65d44d:	vmovdqu YMMWORD PTR [rsp+0x510],ymm0
  65d456:	vmovdqu YMMWORD PTR [rsp+0x4f0],ymm0
  65d45f:	vmovdqu YMMWORD PTR [rsp+0x4d0],ymm0
  65d468:	vmovdqu YMMWORD PTR [rsp+0x4b0],ymm0
  65d471:	vmovdqu YMMWORD PTR [rsp+0x490],ymm0
  65d47a:	mov    QWORD PTR [rsp+0x8],rax
  65d47f:	mov    r12,rax
  65d482:	add    r12,0xe0
  65d489:	xor    r15d,r15d
  65d48c:	mov    r13,QWORD PTR [rip+0x9b255]        # 6f86e8 <_DYNAMIC+0x6d0>
  65d493:	jmp    65d555 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x185>
  65d498:	nop    DWORD PTR [rax+rax*1+0x0]
  65d4a0:	vpternlogq zmm24,zmm9,zmm0,0x96
  65d4a7:	vmovdqu64 zmm0,ZMMWORD PTR [rsp+0x250]
  65d4b2:	vpternlogq zmm0,zmm24,ZMMWORD PTR [rsp+0x450],0x96
  65d4be:	vmovdqu64 zmm3,ZMMWORD PTR [rsp+0x50]
  65d4c9:	vpternlogq zmm3,zmm0,ZMMWORD PTR [rsp+0x190],0x96
  65d4d5:	vpsrlw zmm0,zmm3,0x8
  65d4dc:	vpaddw zmm1,zmm0,zmm0
  65d4e2:	vpbroadcastw zmm2,WORD PTR [rip+0xffffffffff9b4994]        # 11e80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xe32>
  65d4ec:	vpternlogq zmm1,zmm3,zmm2,0x78
  65d4f3:	vpsllw zmm2,zmm0,0x3
  65d4fa:	vpsllw zmm3,zmm0,0x4
  65d501:	vpternlogq zmm3,zmm1,zmm2,0x96
  65d508:	vpsrlw zmm1,zmm3,0x8
  65d50f:	vpaddw zmm2,zmm1,zmm1
  65d515:	vpsllw zmm4,zmm1,0x3
  65d51c:	vpxorq zmm2,zmm2,zmm4
  65d522:	vpsllw zmm4,zmm1,0x4
  65d529:	vpternlogq zmm4,zmm0,zmm2,0x96
  65d530:	vpternlogq zmm4,zmm1,zmm3,0x96
  65d537:	vpmovwb YMMWORD PTR [r12],zmm4
  65d53e:	inc    r15
  65d541:	add    r12,0x100
  65d548:	cmp    r15,0x100
  65d54f:	je     65ed9c <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x19cc>
  65d555:	mov    edi,r15d
  65d558:	vzeroupper 
  65d55b:	call   r13
  65d55e:	mov    BYTE PTR [rsp+r15*1+0x490],al
  65d566:	test   r15b,r15b
  65d569:	js     65e160 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xd90>
  65d56f:	vpxord xmm17,xmm17,xmm17
  65d575:	test   r15b,0x40
  65d579:	je     65dd90 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x9c0>
  65d57f:	vmovdqa64 zmm17,ZMMWORD PTR [rip+0xffffffffff9b3c77]        # 11200 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1b2>
  65d589:	vpxor  xmm13,xmm13,xmm13
  65d58e:	jne    65dd9b <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x9cb>
  65d594:	vpxor  xmm10,xmm10,xmm10
  65d599:	je     65ddb0 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x9e0>
  65d59f:	vmovdqa64 zmm10,ZMMWORD PTR [rip+0xffffffffff9b3cd7]        # 11280 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x232>
  65d5a9:	vpxor  xmm7,xmm7,xmm7
  65d5ad:	jne    65ddba <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x9ea>
  65d5b3:	vpxor  xmm5,xmm5,xmm5
  65d5b7:	je     65ddce <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x9fe>
  65d5bd:	vmovdqa64 zmm5,ZMMWORD PTR [rip+0xffffffffff9b3d39]        # 11300 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x2b2>
  65d5c7:	vpxor  xmm3,xmm3,xmm3
  65d5cb:	jne    65ddd8 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xa08>
  65d5d1:	vpxor  xmm2,xmm2,xmm2
  65d5d5:	je     65ddec <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xa1c>
  65d5db:	vmovdqa64 zmm2,ZMMWORD PTR [rip+0xffffffffff9b3d9b]        # 11380 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x332>
  65d5e5:	vmovdqu64 ZMMWORD PTR [rsp+0x1d0],zmm2
  65d5f0:	vpxor  xmm1,xmm1,xmm1
  65d5f4:	jne    65de01 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xa31>
  65d5fa:	vpxor  xmm0,xmm0,xmm0
  65d5fe:	test   r15b,0x20
  65d602:	je     65de19 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xa49>
  65d608:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b3dee]        # 11400 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x3b2>
  65d612:	vmovdqu64 ZMMWORD PTR [rsp+0x310],zmm0
  65d61d:	vpxor  xmm0,xmm0,xmm0
  65d621:	jne    65de2e <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xa5e>
  65d627:	vmovdqu64 ZMMWORD PTR [rsp+0x390],zmm0
  65d632:	vpxor  xmm0,xmm0,xmm0
  65d636:	je     65de4d <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xa7d>
  65d63c:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b3e3a]        # 11480 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x432>
  65d646:	vmovdqu64 ZMMWORD PTR [rsp+0x410],zmm0
  65d651:	vpxor  xmm0,xmm0,xmm0
  65d655:	jne    65de62 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xa92>
  65d65b:	vmovdqu64 ZMMWORD PTR [rsp+0x90],zmm0
  65d666:	vpxor  xmm0,xmm0,xmm0
  65d66a:	je     65de81 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xab1>
  65d670:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b3e86]        # 11500 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x4b2>
  65d67a:	vmovdqu64 ZMMWORD PTR [rsp+0xd0],zmm0
  65d685:	vpxor  xmm0,xmm0,xmm0
  65d689:	jne    65de96 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xac6>
  65d68f:	vmovdqu64 ZMMWORD PTR [rsp+0x110],zmm0
  65d69a:	vpxor  xmm0,xmm0,xmm0
  65d69e:	je     65deb5 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xae5>
  65d6a4:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b3ed2]        # 11580 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x532>
  65d6ae:	vmovdqu64 ZMMWORD PTR [rsp+0x150],zmm0
  65d6b9:	vmovdqu64 ZMMWORD PTR [rsp+0x50],zmm1
  65d6c4:	vpxor  xmm1,xmm1,xmm1
  65d6c8:	jne    65ded5 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xb05>
  65d6ce:	vpxor  xmm4,xmm4,xmm4
  65d6d2:	test   r15b,0x10
  65d6d6:	je     65deed <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xb1d>
  65d6dc:	vmovdqa64 zmm4,ZMMWORD PTR [rip+0xffffffffff9b3f1a]        # 11600 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x5b2>
  65d6e6:	vpxord xmm29,xmm29,xmm29
  65d6ec:	jne    65def9 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xb29>
  65d6f2:	vpxord xmm26,xmm26,xmm26
  65d6f8:	je     65df0f <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xb3f>
  65d6fe:	vmovdqa64 zmm26,ZMMWORD PTR [rip+0xffffffffff9b3f78]        # 11680 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x632>
  65d708:	vpxord xmm22,xmm22,xmm22
  65d70e:	jne    65df1b <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xb4b>
  65d714:	vpxord xmm19,xmm19,xmm19
  65d71a:	je     65df31 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xb61>
  65d720:	vmovdqa64 zmm19,ZMMWORD PTR [rip+0xffffffffff9b3fd6]        # 11700 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x6b2>
  65d72a:	vpxor  xmm15,xmm15,xmm15
  65d72f:	jne    65df3c <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xb6c>
  65d735:	vpxor  xmm12,xmm12,xmm12
  65d73a:	je     65df51 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xb81>
  65d740:	vmovdqa64 zmm12,ZMMWORD PTR [rip+0xffffffffff9b4036]        # 11780 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x732>
  65d74a:	vpxor  xmm9,xmm9,xmm9
  65d74f:	jne    65df5c <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xb8c>
  65d755:	vpxord xmm20,xmm20,xmm20
  65d75b:	test   r15b,0x8
  65d75f:	je     65df76 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xba6>
  65d765:	vmovdqa64 zmm20,ZMMWORD PTR [rip+0xffffffffff9b4091]        # 11800 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x7b2>
  65d76f:	vmovdqu64 ZMMWORD PTR [rsp+0x250],zmm9
  65d77a:	vpxor  xmm14,xmm14,xmm14
  65d77f:	jne    65df8c <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xbbc>
  65d785:	vpxor  xmm0,xmm0,xmm0
  65d789:	je     65dfa0 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xbd0>
  65d78f:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b40e7]        # 11880 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x832>
  65d799:	vmovdqu64 ZMMWORD PTR [rsp+0x210],zmm0
  65d7a4:	vpxor  xmm0,xmm0,xmm0
  65d7a8:	jne    65dfb5 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xbe5>
  65d7ae:	vmovdqu64 ZMMWORD PTR [rsp+0x290],zmm0
  65d7b9:	vpxor  xmm0,xmm0,xmm0
  65d7bd:	je     65dfd4 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xc04>
  65d7c3:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b4133]        # 11900 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x8b2>
  65d7cd:	vmovdqu64 ZMMWORD PTR [rsp+0x2d0],zmm0
  65d7d8:	vpxor  xmm0,xmm0,xmm0
  65d7dc:	jne    65dfe9 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xc19>
  65d7e2:	vmovdqu64 ZMMWORD PTR [rsp+0x350],zmm0
  65d7ed:	vpxor  xmm0,xmm0,xmm0
  65d7f1:	je     65e008 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xc38>
  65d7f7:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b417f]        # 11980 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x932>
  65d801:	vmovdqu64 ZMMWORD PTR [rsp+0x3d0],zmm0
  65d80c:	vpxor  xmm0,xmm0,xmm0
  65d810:	jne    65e01d <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xc4d>
  65d816:	vpxord xmm30,xmm30,xmm30
  65d81c:	test   r15b,0x4
  65d820:	je     65e037 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xc67>
  65d826:	vmovdqa64 zmm30,ZMMWORD PTR [rip+0xffffffffff9b41d0]        # 11a00 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x9b2>
  65d830:	vpxord xmm25,xmm25,xmm25
  65d836:	jne    65e043 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xc73>
  65d83c:	vpxord xmm21,xmm21,xmm21
  65d842:	je     65e059 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xc89>
  65d848:	vmovdqa64 zmm21,ZMMWORD PTR [rip+0xffffffffff9b422e]        # 11a80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xa32>
  65d852:	vpxord xmm16,xmm16,xmm16
  65d858:	jne    65e065 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xc95>
  65d85e:	vpxor  xmm8,xmm8,xmm8
  65d863:	je     65e07a <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xcaa>
  65d869:	vmovdqa64 zmm8,ZMMWORD PTR [rip+0xffffffffff9b428d]        # 11b00 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xab2>
  65d873:	vpxord xmm31,xmm31,xmm31
  65d879:	jne    65e086 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xcb6>
  65d87f:	vpxord xmm28,xmm28,xmm28
  65d885:	je     65e09c <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xccc>
  65d88b:	vmovdqa64 zmm28,ZMMWORD PTR [rip+0xffffffffff9b42eb]        # 11b80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xb32>
  65d895:	vmovdqu64 ZMMWORD PTR [rsp+0x190],zmm1
  65d8a0:	vpxord xmm24,xmm24,xmm24
  65d8a6:	jne    65e0b3 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xce3>
  65d8ac:	vpxor  xmm1,xmm1,xmm1
  65d8b0:	test   r15b,0x2
  65d8b4:	je     65e0cb <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xcfb>
  65d8ba:	vmovdqa64 zmm1,ZMMWORD PTR [rip+0xffffffffff9b433c]        # 11c00 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xbb2>
  65d8c4:	vmovdqu64 ZMMWORD PTR [rsp+0x450],zmm0
  65d8cf:	vpxor  xmm6,xmm6,xmm6
  65d8d3:	jne    65e0e0 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xd10>
  65d8d9:	vpxor  xmm0,xmm0,xmm0
  65d8dd:	je     65e0f4 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xd24>
  65d8e3:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b4393]        # 11c80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xc32>
  65d8ed:	vpxord xmm27,xmm27,xmm27
  65d8f3:	jne    65e100 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xd30>
  65d8f9:	vpxord xmm23,xmm23,xmm23
  65d8ff:	je     65e116 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xd46>
  65d905:	vmovdqa64 zmm23,ZMMWORD PTR [rip+0xffffffffff9b43f1]        # 11d00 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xcb2>
  65d90f:	vpxord xmm18,xmm18,xmm18
  65d915:	jne    65e122 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xd52>
  65d91b:	vpxor  xmm2,xmm2,xmm2
  65d91f:	je     65e136 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xd66>
  65d925:	vmovdqa64 zmm2,ZMMWORD PTR [rip+0xffffffffff9b4451]        # 11d80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xd32>
  65d92f:	vpxor  xmm9,xmm9,xmm9
  65d934:	jne    65e141 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xd71>
  65d93a:	vpxor  xmm11,xmm11,xmm11
  65d93f:	test   r15b,0x1
  65d943:	je     65d94f <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x57f>
  65d945:	vmovdqa64 zmm11,ZMMWORD PTR [rip+0xffffffffff9b48f1]        # 12240 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x11f2>
  65d94f:	vpternlogq zmm30,zmm1,zmm11,0x96
  65d956:	vpternlogq zmm4,zmm30,zmm20,0x96
  65d95d:	vpternlogq zmm17,zmm4,ZMMWORD PTR [rsp+0x310],0x96
  65d969:	vpsrlw zmm1,zmm17,0x8
  65d970:	vpaddw zmm4,zmm1,zmm1
  65d976:	vpbroadcastw zmm11,WORD PTR [rip+0xffffffffff9b4500]        # 11e80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xe32>
  65d980:	vpternlogq zmm4,zmm17,zmm11,0x78
  65d987:	vpsllw zmm11,zmm1,0x3
  65d98e:	vpsllw zmm17,zmm1,0x4
  65d995:	vpternlogq zmm17,zmm4,zmm11,0x96
  65d99c:	vpsrlw zmm4,zmm17,0x8
  65d9a3:	vpaddw zmm11,zmm4,zmm4
  65d9a9:	vpsllw zmm20,zmm4,0x3
  65d9b0:	vpxorq zmm11,zmm11,zmm20
  65d9b6:	vpsllw zmm20,zmm4,0x4
  65d9bd:	vpternlogq zmm20,zmm1,zmm11,0x96
  65d9c4:	vpternlogq zmm20,zmm4,zmm17,0x96
  65d9cb:	vpmovwb YMMWORD PTR [r12-0xe0],zmm20
  65d9d3:	vpxor  xmm1,xmm1,xmm1
  65d9d7:	je     65d9e3 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x613>
  65d9d9:	vmovdqa64 zmm1,ZMMWORD PTR [rip+0xffffffffff9b489d]        # 12280 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1232>
  65d9e3:	vpternlogq zmm25,zmm6,zmm1,0x96
  65d9ea:	vpternlogq zmm29,zmm25,zmm14,0x96
  65d9f1:	vpternlogq zmm13,zmm29,ZMMWORD PTR [rsp+0x390],0x96
  65d9fd:	vpsrlw zmm1,zmm13,0x8
  65da04:	vpaddw zmm4,zmm1,zmm1
  65da0a:	vpbroadcastw zmm6,WORD PTR [rip+0xffffffffff9b446c]        # 11e80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xe32>
  65da14:	vpternlogq zmm4,zmm13,zmm6,0x78
  65da1b:	vpsllw zmm6,zmm1,0x3
  65da22:	vpsllw zmm11,zmm1,0x4
  65da29:	vpternlogq zmm11,zmm4,zmm6,0x96
  65da30:	vpsrlw zmm4,zmm11,0x8
  65da37:	vpaddw zmm6,zmm4,zmm4
  65da3d:	vpsllw zmm13,zmm4,0x3
  65da44:	vpxorq zmm6,zmm6,zmm13
  65da4a:	vpsllw zmm13,zmm4,0x4
  65da51:	vpternlogq zmm13,zmm1,zmm6,0x96
  65da58:	vpternlogq zmm13,zmm4,zmm11,0x96
  65da5f:	vpmovwb YMMWORD PTR [r12-0xc0],zmm13
  65da67:	vpxor  xmm1,xmm1,xmm1
  65da6b:	je     65da77 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x6a7>
  65da6d:	vmovdqa64 zmm1,ZMMWORD PTR [rip+0xffffffffff9b4849]        # 122c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1272>
  65da77:	vpternlogq zmm21,zmm0,zmm1,0x96
  65da7e:	vpternlogq zmm26,zmm21,ZMMWORD PTR [rsp+0x210],0x96
  65da8a:	vpternlogq zmm10,zmm26,ZMMWORD PTR [rsp+0x410],0x96
  65da96:	vpsrlw zmm0,zmm10,0x8
  65da9d:	vpaddw zmm1,zmm0,zmm0
  65daa3:	vpbroadcastw zmm4,WORD PTR [rip+0xffffffffff9b43d3]        # 11e80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xe32>
  65daad:	vpternlogq zmm1,zmm10,zmm4,0x78
  65dab4:	vpsllw zmm4,zmm0,0x3
  65dabb:	vpsllw zmm6,zmm0,0x4
  65dac2:	vpternlogq zmm6,zmm1,zmm4,0x96
  65dac9:	vpsrlw zmm1,zmm6,0x8
  65dad0:	vpaddw zmm4,zmm1,zmm1
  65dad6:	vpsllw zmm10,zmm1,0x3
  65dadd:	vpxorq zmm4,zmm4,zmm10
  65dae3:	vpsllw zmm10,zmm1,0x4
  65daea:	vpternlogq zmm10,zmm0,zmm4,0x96
  65daf1:	vpternlogq zmm10,zmm1,zmm6,0x96
  65daf8:	vpmovwb YMMWORD PTR [r12-0xa0],zmm10
  65db00:	vpxor  xmm0,xmm0,xmm0
  65db04:	je     65db10 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x740>
  65db06:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b47f0]        # 12300 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x12b2>
  65db10:	vpternlogq zmm16,zmm27,zmm0,0x96
  65db17:	vpternlogq zmm22,zmm16,ZMMWORD PTR [rsp+0x290],0x96
  65db23:	vpternlogq zmm7,zmm22,ZMMWORD PTR [rsp+0x90],0x96
  65db2f:	vpsrlw zmm0,zmm7,0x8
  65db36:	vpaddw zmm1,zmm0,zmm0
  65db3c:	vpbroadcastw zmm4,WORD PTR [rip+0xffffffffff9b433a]        # 11e80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xe32>
  65db46:	vpternlogq zmm1,zmm7,zmm4,0x78
  65db4d:	vpsllw zmm4,zmm0,0x3
  65db54:	vpsllw zmm6,zmm0,0x4
  65db5b:	vpternlogq zmm6,zmm1,zmm4,0x96
  65db62:	vpsrlw zmm1,zmm6,0x8
  65db69:	vpaddw zmm4,zmm1,zmm1
  65db6f:	vpsllw zmm7,zmm1,0x3
  65db76:	vpxorq zmm4,zmm4,zmm7
  65db7c:	vpsllw zmm7,zmm1,0x4
  65db83:	vpternlogq zmm7,zmm0,zmm4,0x96
  65db8a:	vpternlogq zmm7,zmm1,zmm6,0x96
  65db91:	vpmovwb YMMWORD PTR [r12-0x80],zmm7
  65db99:	vpxor  xmm0,xmm0,xmm0
  65db9d:	je     65dba9 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x7d9>
  65db9f:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b4797]        # 12340 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x12f2>
  65dba9:	vpternlogq zmm8,zmm23,zmm0,0x96
  65dbb0:	vpternlogq zmm19,zmm8,ZMMWORD PTR [rsp+0x2d0],0x96
  65dbbc:	vpternlogq zmm5,zmm19,ZMMWORD PTR [rsp+0xd0],0x96
  65dbc8:	vpsrlw zmm0,zmm5,0x8
  65dbcf:	vpaddw zmm1,zmm0,zmm0
  65dbd5:	vpbroadcastw zmm4,WORD PTR [rip+0xffffffffff9b42a1]        # 11e80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xe32>
  65dbdf:	vpternlogq zmm1,zmm5,zmm4,0x78
  65dbe6:	vpsllw zmm4,zmm0,0x3
  65dbed:	vpsllw zmm5,zmm0,0x4
  65dbf4:	vpternlogq zmm5,zmm1,zmm4,0x96
  65dbfb:	vpsrlw zmm1,zmm5,0x8
  65dc02:	vpaddw zmm4,zmm1,zmm1
  65dc08:	vpsllw zmm6,zmm1,0x3
  65dc0f:	vpxorq zmm4,zmm4,zmm6
  65dc15:	vpsllw zmm6,zmm1,0x4
  65dc1c:	vpternlogq zmm6,zmm0,zmm4,0x96
  65dc23:	vpternlogq zmm6,zmm1,zmm5,0x96
  65dc2a:	vpmovwb YMMWORD PTR [r12-0x60],zmm6
  65dc32:	vpxor  xmm0,xmm0,xmm0
  65dc36:	je     65dc42 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x872>
  65dc38:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b473e]        # 12380 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1332>
  65dc42:	vpternlogq zmm31,zmm18,zmm0,0x96
  65dc49:	vpternlogq zmm15,zmm31,ZMMWORD PTR [rsp+0x350],0x96
  65dc55:	vpternlogq zmm3,zmm15,ZMMWORD PTR [rsp+0x110],0x96
  65dc61:	vpsrlw zmm0,zmm3,0x8
  65dc68:	vpaddw zmm1,zmm0,zmm0
  65dc6e:	vpbroadcastw zmm4,WORD PTR [rip+0xffffffffff9b4208]        # 11e80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xe32>
  65dc78:	vpternlogq zmm1,zmm3,zmm4,0x78
  65dc7f:	vpsllw zmm3,zmm0,0x3
  65dc86:	vpsllw zmm4,zmm0,0x4
  65dc8d:	vpternlogq zmm4,zmm1,zmm3,0x96
  65dc94:	vpsrlw zmm1,zmm4,0x8
  65dc9b:	vpaddw zmm3,zmm1,zmm1
  65dca1:	vpsllw zmm5,zmm1,0x3
  65dca8:	vpxorq zmm3,zmm3,zmm5
  65dcae:	vpsllw zmm5,zmm1,0x4
  65dcb5:	vpternlogq zmm5,zmm0,zmm3,0x96
  65dcbc:	vpternlogq zmm5,zmm1,zmm4,0x96
  65dcc3:	vpmovwb YMMWORD PTR [r12-0x40],zmm5
  65dccb:	vpxor  xmm0,xmm0,xmm0
  65dccf:	je     65dcdb <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x90b>
  65dcd1:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b46e5]        # 123c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1372>
  65dcdb:	vpternlogq zmm28,zmm2,zmm0,0x96
  65dce2:	vpternlogq zmm12,zmm28,ZMMWORD PTR [rsp+0x3d0],0x96
  65dcee:	vmovdqu64 zmm2,ZMMWORD PTR [rsp+0x1d0]
  65dcf9:	vpternlogq zmm2,zmm12,ZMMWORD PTR [rsp+0x150],0x96
  65dd05:	vpsrlw zmm0,zmm2,0x8
  65dd0c:	vpaddw zmm1,zmm0,zmm0
  65dd12:	vpbroadcastw zmm3,WORD PTR [rip+0xffffffffff9b4164]        # 11e80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xe32>
  65dd1c:	vpternlogq zmm1,zmm2,zmm3,0x78
  65dd23:	vpsllw zmm2,zmm0,0x3
  65dd2a:	vpsllw zmm3,zmm0,0x4
  65dd31:	vpternlogq zmm3,zmm1,zmm2,0x96
  65dd38:	vpsrlw zmm1,zmm3,0x8
  65dd3f:	vpaddw zmm2,zmm1,zmm1
  65dd45:	vpsllw zmm4,zmm1,0x3
  65dd4c:	vpxorq zmm2,zmm2,zmm4
  65dd52:	vpsllw zmm4,zmm1,0x4
  65dd59:	vpternlogq zmm4,zmm0,zmm2,0x96
  65dd60:	vpternlogq zmm4,zmm1,zmm3,0x96
  65dd67:	vpmovwb YMMWORD PTR [r12-0x20],zmm4
  65dd6f:	vpxor  xmm0,xmm0,xmm0
  65dd73:	je     65d4a0 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xd0>
  65dd79:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b467d]        # 12400 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x13b2>
  65dd83:	jmp    65d4a0 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xd0>
  65dd88:	nop    DWORD PTR [rax+rax*1+0x0]
  65dd90:	vpxor  xmm13,xmm13,xmm13
  65dd95:	je     65d594 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1c4>
  65dd9b:	vmovdqa64 zmm13,ZMMWORD PTR [rip+0xffffffffff9b349b]        # 11240 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1f2>
  65dda5:	vpxor  xmm10,xmm10,xmm10
  65ddaa:	jne    65d59f <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1cf>
  65ddb0:	vpxor  xmm7,xmm7,xmm7
  65ddb4:	je     65d5b3 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1e3>
  65ddba:	vmovdqa64 zmm7,ZMMWORD PTR [rip+0xffffffffff9b34fc]        # 112c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x272>
  65ddc4:	vpxor  xmm5,xmm5,xmm5
  65ddc8:	jne    65d5bd <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1ed>
  65ddce:	vpxor  xmm3,xmm3,xmm3
  65ddd2:	je     65d5d1 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x201>
  65ddd8:	vmovdqa64 zmm3,ZMMWORD PTR [rip+0xffffffffff9b355e]        # 11340 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x2f2>
  65dde2:	vpxor  xmm2,xmm2,xmm2
  65dde6:	jne    65d5db <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x20b>
  65ddec:	vmovdqu64 ZMMWORD PTR [rsp+0x1d0],zmm2
  65ddf7:	vpxor  xmm1,xmm1,xmm1
  65ddfb:	je     65d5fa <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x22a>
  65de01:	vmovdqa64 zmm1,ZMMWORD PTR [rip+0xffffffffff9b35b5]        # 113c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x372>
  65de0b:	vpxor  xmm0,xmm0,xmm0
  65de0f:	test   r15b,0x20
  65de13:	jne    65d608 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x238>
  65de19:	vmovdqu64 ZMMWORD PTR [rsp+0x310],zmm0
  65de24:	vpxor  xmm0,xmm0,xmm0
  65de28:	je     65d627 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x257>
  65de2e:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b3608]        # 11440 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x3f2>
  65de38:	vmovdqu64 ZMMWORD PTR [rsp+0x390],zmm0
  65de43:	vpxor  xmm0,xmm0,xmm0
  65de47:	jne    65d63c <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x26c>
  65de4d:	vmovdqu64 ZMMWORD PTR [rsp+0x410],zmm0
  65de58:	vpxor  xmm0,xmm0,xmm0
  65de5c:	je     65d65b <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x28b>
  65de62:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b3654]        # 114c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x472>
  65de6c:	vmovdqu64 ZMMWORD PTR [rsp+0x90],zmm0
  65de77:	vpxor  xmm0,xmm0,xmm0
  65de7b:	jne    65d670 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x2a0>
  65de81:	vmovdqu64 ZMMWORD PTR [rsp+0xd0],zmm0
  65de8c:	vpxor  xmm0,xmm0,xmm0
  65de90:	je     65d68f <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x2bf>
  65de96:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b36a0]        # 11540 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x4f2>
  65dea0:	vmovdqu64 ZMMWORD PTR [rsp+0x110],zmm0
  65deab:	vpxor  xmm0,xmm0,xmm0
  65deaf:	jne    65d6a4 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x2d4>
  65deb5:	vmovdqu64 ZMMWORD PTR [rsp+0x150],zmm0
  65dec0:	vmovdqu64 ZMMWORD PTR [rsp+0x50],zmm1
  65decb:	vpxor  xmm1,xmm1,xmm1
  65decf:	je     65d6ce <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x2fe>
  65ded5:	vmovdqa64 zmm1,ZMMWORD PTR [rip+0xffffffffff9b36e1]        # 115c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x572>
  65dedf:	vpxor  xmm4,xmm4,xmm4
  65dee3:	test   r15b,0x10
  65dee7:	jne    65d6dc <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x30c>
  65deed:	vpxord xmm29,xmm29,xmm29
  65def3:	je     65d6f2 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x322>
  65def9:	vmovdqa64 zmm29,ZMMWORD PTR [rip+0xffffffffff9b373d]        # 11640 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x5f2>
  65df03:	vpxord xmm26,xmm26,xmm26
  65df09:	jne    65d6fe <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x32e>
  65df0f:	vpxord xmm22,xmm22,xmm22
  65df15:	je     65d714 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x344>
  65df1b:	vmovdqa64 zmm22,ZMMWORD PTR [rip+0xffffffffff9b379b]        # 116c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x672>
  65df25:	vpxord xmm19,xmm19,xmm19
  65df2b:	jne    65d720 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x350>
  65df31:	vpxor  xmm15,xmm15,xmm15
  65df36:	je     65d735 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x365>
  65df3c:	vmovdqa64 zmm15,ZMMWORD PTR [rip+0xffffffffff9b37fa]        # 11740 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x6f2>
  65df46:	vpxor  xmm12,xmm12,xmm12
  65df4b:	jne    65d740 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x370>
  65df51:	vpxor  xmm9,xmm9,xmm9
  65df56:	je     65d755 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x385>
  65df5c:	vmovdqa64 zmm9,ZMMWORD PTR [rip+0xffffffffff9b385a]        # 117c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x772>
  65df66:	vpxord xmm20,xmm20,xmm20
  65df6c:	test   r15b,0x8
  65df70:	jne    65d765 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x395>
  65df76:	vmovdqu64 ZMMWORD PTR [rsp+0x250],zmm9
  65df81:	vpxor  xmm14,xmm14,xmm14
  65df86:	je     65d785 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x3b5>
  65df8c:	vmovdqa64 zmm14,ZMMWORD PTR [rip+0xffffffffff9b38aa]        # 11840 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x7f2>
  65df96:	vpxor  xmm0,xmm0,xmm0
  65df9a:	jne    65d78f <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x3bf>
  65dfa0:	vmovdqu64 ZMMWORD PTR [rsp+0x210],zmm0
  65dfab:	vpxor  xmm0,xmm0,xmm0
  65dfaf:	je     65d7ae <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x3de>
  65dfb5:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b3901]        # 118c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x872>
  65dfbf:	vmovdqu64 ZMMWORD PTR [rsp+0x290],zmm0
  65dfca:	vpxor  xmm0,xmm0,xmm0
  65dfce:	jne    65d7c3 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x3f3>
  65dfd4:	vmovdqu64 ZMMWORD PTR [rsp+0x2d0],zmm0
  65dfdf:	vpxor  xmm0,xmm0,xmm0
  65dfe3:	je     65d7e2 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x412>
  65dfe9:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b394d]        # 11940 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x8f2>
  65dff3:	vmovdqu64 ZMMWORD PTR [rsp+0x350],zmm0
  65dffe:	vpxor  xmm0,xmm0,xmm0
  65e002:	jne    65d7f7 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x427>
  65e008:	vmovdqu64 ZMMWORD PTR [rsp+0x3d0],zmm0
  65e013:	vpxor  xmm0,xmm0,xmm0
  65e017:	je     65d816 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x446>
  65e01d:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b3999]        # 119c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x972>
  65e027:	vpxord xmm30,xmm30,xmm30
  65e02d:	test   r15b,0x4
  65e031:	jne    65d826 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x456>
  65e037:	vpxord xmm25,xmm25,xmm25
  65e03d:	je     65d83c <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x46c>
  65e043:	vmovdqa64 zmm25,ZMMWORD PTR [rip+0xffffffffff9b39f3]        # 11a40 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x9f2>
  65e04d:	vpxord xmm21,xmm21,xmm21
  65e053:	jne    65d848 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x478>
  65e059:	vpxord xmm16,xmm16,xmm16
  65e05f:	je     65d85e <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x48e>
  65e065:	vmovdqa64 zmm16,ZMMWORD PTR [rip+0xffffffffff9b3a51]        # 11ac0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xa72>
  65e06f:	vpxor  xmm8,xmm8,xmm8
  65e074:	jne    65d869 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x499>
  65e07a:	vpxord xmm31,xmm31,xmm31
  65e080:	je     65d87f <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x4af>
  65e086:	vmovdqa64 zmm31,ZMMWORD PTR [rip+0xffffffffff9b3ab0]        # 11b40 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xaf2>
  65e090:	vpxord xmm28,xmm28,xmm28
  65e096:	jne    65d88b <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x4bb>
  65e09c:	vmovdqu64 ZMMWORD PTR [rsp+0x190],zmm1
  65e0a7:	vpxord xmm24,xmm24,xmm24
  65e0ad:	je     65d8ac <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x4dc>
  65e0b3:	vmovdqa64 zmm24,ZMMWORD PTR [rip+0xffffffffff9b3b03]        # 11bc0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xb72>
  65e0bd:	vpxor  xmm1,xmm1,xmm1
  65e0c1:	test   r15b,0x2
  65e0c5:	jne    65d8ba <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x4ea>
  65e0cb:	vmovdqu64 ZMMWORD PTR [rsp+0x450],zmm0
  65e0d6:	vpxor  xmm6,xmm6,xmm6
  65e0da:	je     65d8d9 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x509>
  65e0e0:	vmovdqa64 zmm6,ZMMWORD PTR [rip+0xffffffffff9b3b56]        # 11c40 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xbf2>
  65e0ea:	vpxor  xmm0,xmm0,xmm0
  65e0ee:	jne    65d8e3 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x513>
  65e0f4:	vpxord xmm27,xmm27,xmm27
  65e0fa:	je     65d8f9 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x529>
  65e100:	vmovdqa64 zmm27,ZMMWORD PTR [rip+0xffffffffff9b3bb6]        # 11cc0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xc72>
  65e10a:	vpxord xmm23,xmm23,xmm23
  65e110:	jne    65d905 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x535>
  65e116:	vpxord xmm18,xmm18,xmm18
  65e11c:	je     65d91b <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x54b>
  65e122:	vmovdqa64 zmm18,ZMMWORD PTR [rip+0xffffffffff9b3c14]        # 11d40 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xcf2>
  65e12c:	vpxor  xmm2,xmm2,xmm2
  65e130:	jne    65d925 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x555>
  65e136:	vpxor  xmm9,xmm9,xmm9
  65e13b:	je     65d93a <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x56a>
  65e141:	vmovdqa64 zmm9,ZMMWORD PTR [rip+0xffffffffff9b3c75]        # 11dc0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xd72>
  65e14b:	vpxor  xmm11,xmm11,xmm11
  65e150:	test   r15b,0x1
  65e154:	jne    65d945 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x575>
  65e15a:	jmp    65d94f <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x57f>
  65e15f:	nop
  65e160:	vpxor  xmm0,xmm0,xmm0
  65e164:	vmovdqu64 ZMMWORD PTR [rsp+0x50],zmm0
  65e16f:	vpxord xmm17,xmm17,xmm17
  65e175:	test   r15b,0x40
  65e179:	je     65e9d0 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1600>
  65e17f:	vmovdqa64 zmm17,ZMMWORD PTR [rip+0xffffffffff9b3077]        # 11200 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1b2>
  65e189:	vpxor  xmm13,xmm13,xmm13
  65e18e:	jne    65e9db <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x160b>
  65e194:	vpxor  xmm10,xmm10,xmm10
  65e199:	je     65e9f0 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1620>
  65e19f:	vmovdqa64 zmm10,ZMMWORD PTR [rip+0xffffffffff9b30d7]        # 11280 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x232>
  65e1a9:	vpxor  xmm7,xmm7,xmm7
  65e1ad:	jne    65e9fa <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x162a>
  65e1b3:	vpxor  xmm5,xmm5,xmm5
  65e1b7:	je     65ea0e <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x163e>
  65e1bd:	vmovdqa64 zmm5,ZMMWORD PTR [rip+0xffffffffff9b3139]        # 11300 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x2b2>
  65e1c7:	vpxor  xmm3,xmm3,xmm3
  65e1cb:	jne    65ea18 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1648>
  65e1d1:	vpxor  xmm2,xmm2,xmm2
  65e1d5:	je     65ea2c <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x165c>
  65e1db:	vmovdqa64 zmm2,ZMMWORD PTR [rip+0xffffffffff9b319b]        # 11380 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x332>
  65e1e5:	jne    65ea32 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1662>
  65e1eb:	vmovdqu64 ZMMWORD PTR [rsp+0x190],zmm0
  65e1f6:	vpxor  xmm0,xmm0,xmm0
  65e1fa:	test   r15b,0x20
  65e1fe:	je     65ea55 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1685>
  65e204:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b31f2]        # 11400 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x3b2>
  65e20e:	vmovdqu64 ZMMWORD PTR [rsp+0x2d0],zmm0
  65e219:	vpxor  xmm0,xmm0,xmm0
  65e21d:	jne    65ea6a <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x169a>
  65e223:	vmovdqu64 ZMMWORD PTR [rsp+0x350],zmm0
  65e22e:	vpxor  xmm0,xmm0,xmm0
  65e232:	je     65ea89 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x16b9>
  65e238:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b323e]        # 11480 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x432>
  65e242:	vmovdqu64 ZMMWORD PTR [rsp+0x3d0],zmm0
  65e24d:	vpxor  xmm0,xmm0,xmm0
  65e251:	jne    65ea9e <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x16ce>
  65e257:	vmovdqu64 ZMMWORD PTR [rsp+0x450],zmm0
  65e262:	vpxor  xmm0,xmm0,xmm0
  65e266:	je     65eabd <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x16ed>
  65e26c:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b328a]        # 11500 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x4b2>
  65e276:	vmovdqu64 ZMMWORD PTR [rsp+0x90],zmm0
  65e281:	vpxor  xmm0,xmm0,xmm0
  65e285:	jne    65ead2 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1702>
  65e28b:	vmovdqu64 ZMMWORD PTR [rsp+0xd0],zmm0
  65e296:	vpxor  xmm0,xmm0,xmm0
  65e29a:	je     65eaf1 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1721>
  65e2a0:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b32d6]        # 11580 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x532>
  65e2aa:	vmovdqu64 ZMMWORD PTR [rsp+0x110],zmm0
  65e2b5:	vpxor  xmm0,xmm0,xmm0
  65e2b9:	jne    65eb06 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1736>
  65e2bf:	vmovdqu64 ZMMWORD PTR [rsp+0x150],zmm0
  65e2ca:	vpxor  xmm0,xmm0,xmm0
  65e2ce:	test   r15b,0x10
  65e2d2:	je     65eb29 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1759>
  65e2d8:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b331e]        # 11600 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x5b2>
  65e2e2:	vpxord xmm29,xmm29,xmm29
  65e2e8:	jne    65eb35 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1765>
  65e2ee:	vpxord xmm26,xmm26,xmm26
  65e2f4:	je     65eb4b <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x177b>
  65e2fa:	vmovdqa64 zmm26,ZMMWORD PTR [rip+0xffffffffff9b337c]        # 11680 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x632>
  65e304:	vpxord xmm22,xmm22,xmm22
  65e30a:	jne    65eb57 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1787>
  65e310:	vpxord xmm19,xmm19,xmm19
  65e316:	je     65eb6d <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x179d>
  65e31c:	vmovdqa64 zmm19,ZMMWORD PTR [rip+0xffffffffff9b33da]        # 11700 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x6b2>
  65e326:	vpxor  xmm15,xmm15,xmm15
  65e32b:	jne    65eb78 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x17a8>
  65e331:	vpxor  xmm12,xmm12,xmm12
  65e336:	je     65eb8d <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x17bd>
  65e33c:	vmovdqa64 zmm12,ZMMWORD PTR [rip+0xffffffffff9b343a]        # 11780 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x732>
  65e346:	vpxor  xmm9,xmm9,xmm9
  65e34b:	jne    65eb98 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x17c8>
  65e351:	vpxord xmm18,xmm18,xmm18
  65e357:	test   r15b,0x8
  65e35b:	je     65ebb2 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x17e2>
  65e361:	vmovdqa64 zmm18,ZMMWORD PTR [rip+0xffffffffff9b3495]        # 11800 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x7b2>
  65e36b:	vpxor  xmm11,xmm11,xmm11
  65e370:	jne    65ebbd <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x17ed>
  65e376:	vpxor  xmm1,xmm1,xmm1
  65e37a:	je     65ebd1 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1801>
  65e380:	vmovdqa64 zmm1,ZMMWORD PTR [rip+0xffffffffff9b34f6]        # 11880 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x832>
  65e38a:	vmovdqu64 ZMMWORD PTR [rsp+0x210],zmm1
  65e395:	vpxor  xmm1,xmm1,xmm1
  65e399:	jne    65ebe6 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1816>
  65e39f:	vmovdqu64 ZMMWORD PTR [rsp+0x250],zmm1
  65e3aa:	vpxor  xmm1,xmm1,xmm1
  65e3ae:	je     65ec05 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1835>
  65e3b4:	vmovdqa64 zmm1,ZMMWORD PTR [rip+0xffffffffff9b3542]        # 11900 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x8b2>
  65e3be:	vmovdqu64 ZMMWORD PTR [rsp+0x290],zmm1
  65e3c9:	vpxor  xmm1,xmm1,xmm1
  65e3cd:	jne    65ec1a <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x184a>
  65e3d3:	vmovdqu64 ZMMWORD PTR [rsp+0x310],zmm1
  65e3de:	vpxor  xmm1,xmm1,xmm1
  65e3e2:	je     65ec39 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1869>
  65e3e8:	vmovdqa64 zmm1,ZMMWORD PTR [rip+0xffffffffff9b358e]        # 11980 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x932>
  65e3f2:	vmovdqu64 ZMMWORD PTR [rsp+0x390],zmm1
  65e3fd:	vpxor  xmm1,xmm1,xmm1
  65e401:	jne    65ec4e <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x187e>
  65e407:	vpxord xmm27,xmm27,xmm27
  65e40d:	test   r15b,0x4
  65e411:	je     65ec68 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1898>
  65e417:	vmovdqa64 zmm27,ZMMWORD PTR [rip+0xffffffffff9b35df]        # 11a00 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x9b2>
  65e421:	vpxord xmm23,xmm23,xmm23
  65e427:	jne    65ec74 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x18a4>
  65e42d:	vpxord xmm20,xmm20,xmm20
  65e433:	je     65ec8a <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x18ba>
  65e439:	vmovdqa64 zmm20,ZMMWORD PTR [rip+0xffffffffff9b363d]        # 11a80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xa32>
  65e443:	vpxor  xmm14,xmm14,xmm14
  65e448:	jne    65ec95 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x18c5>
  65e44e:	vpxor  xmm6,xmm6,xmm6
  65e452:	je     65eca9 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x18d9>
  65e458:	vmovdqa64 zmm6,ZMMWORD PTR [rip+0xffffffffff9b369e]        # 11b00 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xab2>
  65e462:	vpxord xmm31,xmm31,xmm31
  65e468:	jne    65ecb5 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x18e5>
  65e46e:	vpxord xmm28,xmm28,xmm28
  65e474:	je     65eccb <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x18fb>
  65e47a:	vmovdqa64 zmm28,ZMMWORD PTR [rip+0xffffffffff9b36fc]        # 11b80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xb32>
  65e484:	vmovdqu64 ZMMWORD PTR [rsp+0x410],zmm1
  65e48f:	vpxord xmm24,xmm24,xmm24
  65e495:	jne    65ece2 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1912>
  65e49b:	vpxor  xmm1,xmm1,xmm1
  65e49f:	test   r15b,0x2
  65e4a3:	je     65ecfa <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x192a>
  65e4a9:	vmovdqa64 zmm1,ZMMWORD PTR [rip+0xffffffffff9b374d]        # 11c00 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xbb2>
  65e4b3:	vpxor  xmm4,xmm4,xmm4
  65e4b7:	jne    65ed04 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1934>
  65e4bd:	vpxord xmm30,xmm30,xmm30
  65e4c3:	je     65ed1a <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x194a>
  65e4c9:	vmovdqa64 zmm30,ZMMWORD PTR [rip+0xffffffffff9b37ad]        # 11c80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xc32>
  65e4d3:	vpxord xmm25,xmm25,xmm25
  65e4d9:	jne    65ed26 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1956>
  65e4df:	vpxord xmm21,xmm21,xmm21
  65e4e5:	je     65ed3c <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x196c>
  65e4eb:	vmovdqa64 zmm21,ZMMWORD PTR [rip+0xffffffffff9b380b]        # 11d00 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xcb2>
  65e4f5:	vpxord xmm16,xmm16,xmm16
  65e4fb:	jne    65ed48 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1978>
  65e501:	vxorps xmm8,xmm8,xmm8
  65e506:	je     65ed5d <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x198d>
  65e50c:	vmovaps zmm8,ZMMWORD PTR [rip+0xffffffffff9b386a]        # 11d80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xd32>
  65e516:	vmovups ZMMWORD PTR [rsp+0x1d0],zmm8
  65e521:	jne    65ed6e <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x199e>
  65e527:	vmovdqa64 zmm8,ZMMWORD PTR [rip+0xffffffffff9b38cf]        # 11e00 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xdb2>
  65e531:	test   r15b,0x1
  65e535:	je     65e541 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1171>
  65e537:	vmovdqa64 zmm8,ZMMWORD PTR [rip+0xffffffffff9b38ff]        # 11e40 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xdf2>
  65e541:	vpternlogq zmm27,zmm8,zmm1,0x96
  65e548:	vpternlogq zmm0,zmm27,zmm18,0x96
  65e54f:	vpternlogq zmm17,zmm0,ZMMWORD PTR [rsp+0x2d0],0x96
  65e55b:	vpsrlw zmm0,zmm17,0x8
  65e562:	vpaddw zmm1,zmm0,zmm0
  65e568:	vpbroadcastw zmm8,WORD PTR [rip+0xffffffffff9b390e]        # 11e80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xe32>
  65e572:	vpternlogq zmm1,zmm17,zmm8,0x78
  65e579:	vpsllw zmm8,zmm0,0x3
  65e580:	vpsllw zmm17,zmm0,0x4
  65e587:	vpternlogq zmm17,zmm1,zmm8,0x96
  65e58e:	vpsrlw zmm1,zmm17,0x8
  65e595:	vpaddw zmm8,zmm1,zmm1
  65e59b:	vpsllw zmm18,zmm1,0x3
  65e5a2:	vpxorq zmm8,zmm8,zmm18
  65e5a8:	vpsllw zmm18,zmm1,0x4
  65e5af:	vpternlogq zmm18,zmm0,zmm8,0x96
  65e5b6:	vpternlogq zmm18,zmm1,zmm17,0x96
  65e5bd:	vpmovwb YMMWORD PTR [r12-0xe0],zmm18
  65e5c5:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b38f1]        # 11ec0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xe72>
  65e5cf:	je     65e5db <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x120b>
  65e5d1:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b3925]        # 11f00 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xeb2>
  65e5db:	vpternlogq zmm23,zmm0,zmm4,0x96
  65e5e2:	vpternlogq zmm29,zmm23,zmm11,0x96
  65e5e9:	vpternlogq zmm13,zmm29,ZMMWORD PTR [rsp+0x350],0x96
  65e5f5:	vpsrlw zmm0,zmm13,0x8
  65e5fc:	vpaddw zmm1,zmm0,zmm0
  65e602:	vpbroadcastw zmm4,WORD PTR [rip+0xffffffffff9b3874]        # 11e80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xe32>
  65e60c:	vpternlogq zmm1,zmm13,zmm4,0x78
  65e613:	vpsllw zmm4,zmm0,0x3
  65e61a:	vpsllw zmm8,zmm0,0x4
  65e621:	vpternlogq zmm8,zmm1,zmm4,0x96
  65e628:	vpsrlw zmm1,zmm8,0x8
  65e62f:	vpaddw zmm4,zmm1,zmm1
  65e635:	vpsllw zmm11,zmm1,0x3
  65e63c:	vpxorq zmm4,zmm4,zmm11
  65e642:	vpsllw zmm11,zmm1,0x4
  65e649:	vpternlogq zmm11,zmm0,zmm4,0x96
  65e650:	vpternlogq zmm11,zmm1,zmm8,0x96
  65e657:	vpmovwb YMMWORD PTR [r12-0xc0],zmm11
  65e65f:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b38d7]        # 11f40 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xef2>
  65e669:	je     65e675 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x12a5>
  65e66b:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b390b]        # 11f80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xf32>
  65e675:	vpternlogq zmm20,zmm0,zmm30,0x96
  65e67c:	vpternlogq zmm26,zmm20,ZMMWORD PTR [rsp+0x210],0x96
  65e688:	vpternlogq zmm10,zmm26,ZMMWORD PTR [rsp+0x3d0],0x96
  65e694:	vpsrlw zmm0,zmm10,0x8
  65e69b:	vpaddw zmm1,zmm0,zmm0
  65e6a1:	vpbroadcastw zmm4,WORD PTR [rip+0xffffffffff9b37d5]        # 11e80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xe32>
  65e6ab:	vpternlogq zmm1,zmm10,zmm4,0x78
  65e6b2:	vpsllw zmm4,zmm0,0x3
  65e6b9:	vpsllw zmm8,zmm0,0x4
  65e6c0:	vpternlogq zmm8,zmm1,zmm4,0x96
  65e6c7:	vpsrlw zmm1,zmm8,0x8
  65e6ce:	vpaddw zmm4,zmm1,zmm1
  65e6d4:	vpsllw zmm10,zmm1,0x3
  65e6db:	vpxorq zmm4,zmm4,zmm10
  65e6e1:	vpsllw zmm10,zmm1,0x4
  65e6e8:	vpternlogq zmm10,zmm0,zmm4,0x96
  65e6ef:	vpternlogq zmm10,zmm1,zmm8,0x96
  65e6f6:	vpmovwb YMMWORD PTR [r12-0xa0],zmm10
  65e6fe:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b38b8]        # 11fc0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xf72>
  65e708:	je     65e714 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1344>
  65e70a:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b38ec]        # 12000 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xfb2>
  65e714:	vpternlogq zmm14,zmm0,zmm25,0x96
  65e71b:	vpternlogq zmm22,zmm14,ZMMWORD PTR [rsp+0x250],0x96
  65e727:	vpternlogq zmm7,zmm22,ZMMWORD PTR [rsp+0x450],0x96
  65e733:	vpsrlw zmm0,zmm7,0x8
  65e73a:	vpaddw zmm1,zmm0,zmm0
  65e740:	vpbroadcastw zmm4,WORD PTR [rip+0xffffffffff9b3736]        # 11e80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xe32>
  65e74a:	vpternlogq zmm1,zmm7,zmm4,0x78
  65e751:	vpsllw zmm4,zmm0,0x3
  65e758:	vpsllw zmm7,zmm0,0x4
  65e75f:	vpternlogq zmm7,zmm1,zmm4,0x96
  65e766:	vpsrlw zmm1,zmm7,0x8
  65e76d:	vpaddw zmm4,zmm1,zmm1
  65e773:	vpsllw zmm8,zmm1,0x3
  65e77a:	vpxorq zmm4,zmm4,zmm8
  65e780:	vpsllw zmm8,zmm1,0x4
  65e787:	vpternlogq zmm8,zmm0,zmm4,0x96
  65e78e:	vpternlogq zmm8,zmm1,zmm7,0x96
  65e795:	vpmovwb YMMWORD PTR [r12-0x80],zmm8
  65e79d:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b3899]        # 12040 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xff2>
  65e7a7:	je     65e7b3 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x13e3>
  65e7a9:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b38cd]        # 12080 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1032>
  65e7b3:	vpternlogq zmm6,zmm0,zmm21,0x96
  65e7ba:	vpternlogq zmm19,zmm6,ZMMWORD PTR [rsp+0x290],0x96
  65e7c6:	vpternlogq zmm5,zmm19,ZMMWORD PTR [rsp+0x90],0x96
  65e7d2:	vpsrlw zmm0,zmm5,0x8
  65e7d9:	vpaddw zmm1,zmm0,zmm0
  65e7df:	vpbroadcastw zmm4,WORD PTR [rip+0xffffffffff9b3697]        # 11e80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xe32>
  65e7e9:	vpternlogq zmm1,zmm5,zmm4,0x78
  65e7f0:	vpsllw zmm4,zmm0,0x3
  65e7f7:	vpsllw zmm5,zmm0,0x4
  65e7fe:	vpternlogq zmm5,zmm1,zmm4,0x96
  65e805:	vpsrlw zmm1,zmm5,0x8
  65e80c:	vpaddw zmm4,zmm1,zmm1
  65e812:	vpsllw zmm6,zmm1,0x3
  65e819:	vpxorq zmm4,zmm4,zmm6
  65e81f:	vpsllw zmm6,zmm1,0x4
  65e826:	vpternlogq zmm6,zmm0,zmm4,0x96
  65e82d:	vpternlogq zmm6,zmm1,zmm5,0x96
  65e834:	vpmovwb YMMWORD PTR [r12-0x60],zmm6
  65e83c:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b387a]        # 120c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1072>
  65e846:	vmovdqu64 zmm6,ZMMWORD PTR [rsp+0x1d0]
  65e851:	je     65e85d <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x148d>
  65e853:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b38a3]        # 12100 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x10b2>
  65e85d:	vpternlogq zmm31,zmm0,zmm16,0x96
  65e864:	vpternlogq zmm15,zmm31,ZMMWORD PTR [rsp+0x310],0x96
  65e870:	vpternlogq zmm3,zmm15,ZMMWORD PTR [rsp+0xd0],0x96
  65e87c:	vpsrlw zmm0,zmm3,0x8
  65e883:	vpaddw zmm1,zmm0,zmm0
  65e889:	vpbroadcastw zmm4,WORD PTR [rip+0xffffffffff9b35ed]        # 11e80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xe32>
  65e893:	vpternlogq zmm1,zmm3,zmm4,0x78
  65e89a:	vpsllw zmm3,zmm0,0x3
  65e8a1:	vpsllw zmm4,zmm0,0x4
  65e8a8:	vpternlogq zmm4,zmm1,zmm3,0x96
  65e8af:	vpsrlw zmm1,zmm4,0x8
  65e8b6:	vpaddw zmm3,zmm1,zmm1
  65e8bc:	vpsllw zmm5,zmm1,0x3
  65e8c3:	vpxorq zmm3,zmm3,zmm5
  65e8c9:	vpsllw zmm5,zmm1,0x4
  65e8d0:	vpternlogq zmm5,zmm0,zmm3,0x96
  65e8d7:	vpternlogq zmm5,zmm1,zmm4,0x96
  65e8de:	vpmovwb YMMWORD PTR [r12-0x40],zmm5
  65e8e6:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b3850]        # 12140 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x10f2>
  65e8f0:	je     65e8fc <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x152c>
  65e8f2:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b3884]        # 12180 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1132>
  65e8fc:	vpternlogq zmm28,zmm0,zmm6,0x96
  65e903:	vpternlogq zmm12,zmm28,ZMMWORD PTR [rsp+0x390],0x96
  65e90f:	vpternlogq zmm2,zmm12,ZMMWORD PTR [rsp+0x110],0x96
  65e91b:	vpsrlw zmm0,zmm2,0x8
  65e922:	vpaddw zmm1,zmm0,zmm0
  65e928:	vpbroadcastw zmm3,WORD PTR [rip+0xffffffffff9b354e]        # 11e80 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xe32>
  65e932:	vpternlogq zmm1,zmm2,zmm3,0x78
  65e939:	vpsllw zmm2,zmm0,0x3
  65e940:	vpsllw zmm3,zmm0,0x4
  65e947:	vpternlogq zmm3,zmm1,zmm2,0x96
  65e94e:	vpsrlw zmm1,zmm3,0x8
  65e955:	vpaddw zmm2,zmm1,zmm1
  65e95b:	vpsllw zmm4,zmm1,0x3
  65e962:	vpxorq zmm2,zmm2,zmm4
  65e968:	vpsllw zmm4,zmm1,0x4
  65e96f:	vpternlogq zmm4,zmm0,zmm2,0x96
  65e976:	vpternlogq zmm4,zmm1,zmm3,0x96
  65e97d:	vpmovwb YMMWORD PTR [r12-0x20],zmm4
  65e985:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b3831]        # 121c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1172>
  65e98f:	je     65e99b <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x15cb>
  65e991:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b3865]        # 12200 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x11b2>
  65e99b:	vpternlogq zmm24,zmm0,ZMMWORD PTR [rsp+0x50],0x96
  65e9a7:	vpternlogq zmm9,zmm24,ZMMWORD PTR [rsp+0x410],0x96
  65e9b3:	vmovdqu64 zmm3,ZMMWORD PTR [rsp+0x190]
  65e9be:	vpternlogq zmm3,zmm9,ZMMWORD PTR [rsp+0x150],0x96
  65e9ca:	jmp    65d4d5 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x105>
  65e9cf:	nop
  65e9d0:	vpxor  xmm13,xmm13,xmm13
  65e9d5:	je     65e194 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xdc4>
  65e9db:	vmovdqa64 zmm13,ZMMWORD PTR [rip+0xffffffffff9b285b]        # 11240 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1f2>
  65e9e5:	vpxor  xmm10,xmm10,xmm10
  65e9ea:	jne    65e19f <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xdcf>
  65e9f0:	vpxor  xmm7,xmm7,xmm7
  65e9f4:	je     65e1b3 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xde3>
  65e9fa:	vmovdqa64 zmm7,ZMMWORD PTR [rip+0xffffffffff9b28bc]        # 112c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x272>
  65ea04:	vpxor  xmm5,xmm5,xmm5
  65ea08:	jne    65e1bd <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xded>
  65ea0e:	vpxor  xmm3,xmm3,xmm3
  65ea12:	je     65e1d1 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xe01>
  65ea18:	vmovdqa64 zmm3,ZMMWORD PTR [rip+0xffffffffff9b291e]        # 11340 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x2f2>
  65ea22:	vpxor  xmm2,xmm2,xmm2
  65ea26:	jne    65e1db <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xe0b>
  65ea2c:	je     65e1eb <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xe1b>
  65ea32:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b2984]        # 113c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x372>
  65ea3c:	vmovdqu64 ZMMWORD PTR [rsp+0x190],zmm0
  65ea47:	vpxor  xmm0,xmm0,xmm0
  65ea4b:	test   r15b,0x20
  65ea4f:	jne    65e204 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xe34>
  65ea55:	vmovdqu64 ZMMWORD PTR [rsp+0x2d0],zmm0
  65ea60:	vpxor  xmm0,xmm0,xmm0
  65ea64:	je     65e223 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xe53>
  65ea6a:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b29cc]        # 11440 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x3f2>
  65ea74:	vmovdqu64 ZMMWORD PTR [rsp+0x350],zmm0
  65ea7f:	vpxor  xmm0,xmm0,xmm0
  65ea83:	jne    65e238 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xe68>
  65ea89:	vmovdqu64 ZMMWORD PTR [rsp+0x3d0],zmm0
  65ea94:	vpxor  xmm0,xmm0,xmm0
  65ea98:	je     65e257 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xe87>
  65ea9e:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b2a18]        # 114c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x472>
  65eaa8:	vmovdqu64 ZMMWORD PTR [rsp+0x450],zmm0
  65eab3:	vpxor  xmm0,xmm0,xmm0
  65eab7:	jne    65e26c <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xe9c>
  65eabd:	vmovdqu64 ZMMWORD PTR [rsp+0x90],zmm0
  65eac8:	vpxor  xmm0,xmm0,xmm0
  65eacc:	je     65e28b <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xebb>
  65ead2:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b2a64]        # 11540 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x4f2>
  65eadc:	vmovdqu64 ZMMWORD PTR [rsp+0xd0],zmm0
  65eae7:	vpxor  xmm0,xmm0,xmm0
  65eaeb:	jne    65e2a0 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xed0>
  65eaf1:	vmovdqu64 ZMMWORD PTR [rsp+0x110],zmm0
  65eafc:	vpxor  xmm0,xmm0,xmm0
  65eb00:	je     65e2bf <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xeef>
  65eb06:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9b2ab0]        # 115c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x572>
  65eb10:	vmovdqu64 ZMMWORD PTR [rsp+0x150],zmm0
  65eb1b:	vpxor  xmm0,xmm0,xmm0
  65eb1f:	test   r15b,0x10
  65eb23:	jne    65e2d8 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xf08>
  65eb29:	vpxord xmm29,xmm29,xmm29
  65eb2f:	je     65e2ee <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xf1e>
  65eb35:	vmovdqa64 zmm29,ZMMWORD PTR [rip+0xffffffffff9b2b01]        # 11640 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x5f2>
  65eb3f:	vpxord xmm26,xmm26,xmm26
  65eb45:	jne    65e2fa <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xf2a>
  65eb4b:	vpxord xmm22,xmm22,xmm22
  65eb51:	je     65e310 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xf40>
  65eb57:	vmovdqa64 zmm22,ZMMWORD PTR [rip+0xffffffffff9b2b5f]        # 116c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x672>
  65eb61:	vpxord xmm19,xmm19,xmm19
  65eb67:	jne    65e31c <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xf4c>
  65eb6d:	vpxor  xmm15,xmm15,xmm15
  65eb72:	je     65e331 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xf61>
  65eb78:	vmovdqa64 zmm15,ZMMWORD PTR [rip+0xffffffffff9b2bbe]        # 11740 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x6f2>
  65eb82:	vpxor  xmm12,xmm12,xmm12
  65eb87:	jne    65e33c <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xf6c>
  65eb8d:	vpxor  xmm9,xmm9,xmm9
  65eb92:	je     65e351 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xf81>
  65eb98:	vmovdqa64 zmm9,ZMMWORD PTR [rip+0xffffffffff9b2c1e]        # 117c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x772>
  65eba2:	vpxord xmm18,xmm18,xmm18
  65eba8:	test   r15b,0x8
  65ebac:	jne    65e361 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xf91>
  65ebb2:	vpxor  xmm11,xmm11,xmm11
  65ebb7:	je     65e376 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xfa6>
  65ebbd:	vmovdqa64 zmm11,ZMMWORD PTR [rip+0xffffffffff9b2c79]        # 11840 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x7f2>
  65ebc7:	vpxor  xmm1,xmm1,xmm1
  65ebcb:	jne    65e380 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xfb0>
  65ebd1:	vmovdqu64 ZMMWORD PTR [rsp+0x210],zmm1
  65ebdc:	vpxor  xmm1,xmm1,xmm1
  65ebe0:	je     65e39f <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xfcf>
  65ebe6:	vmovdqa64 zmm1,ZMMWORD PTR [rip+0xffffffffff9b2cd0]        # 118c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x872>
  65ebf0:	vmovdqu64 ZMMWORD PTR [rsp+0x250],zmm1
  65ebfb:	vpxor  xmm1,xmm1,xmm1
  65ebff:	jne    65e3b4 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0xfe4>
  65ec05:	vmovdqu64 ZMMWORD PTR [rsp+0x290],zmm1
  65ec10:	vpxor  xmm1,xmm1,xmm1
  65ec14:	je     65e3d3 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1003>
  65ec1a:	vmovdqa64 zmm1,ZMMWORD PTR [rip+0xffffffffff9b2d1c]        # 11940 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x8f2>
  65ec24:	vmovdqu64 ZMMWORD PTR [rsp+0x310],zmm1
  65ec2f:	vpxor  xmm1,xmm1,xmm1
  65ec33:	jne    65e3e8 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1018>
  65ec39:	vmovdqu64 ZMMWORD PTR [rsp+0x390],zmm1
  65ec44:	vpxor  xmm1,xmm1,xmm1
  65ec48:	je     65e407 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1037>
  65ec4e:	vmovdqa64 zmm1,ZMMWORD PTR [rip+0xffffffffff9b2d68]        # 119c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x972>
  65ec58:	vpxord xmm27,xmm27,xmm27
  65ec5e:	test   r15b,0x4
  65ec62:	jne    65e417 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1047>
  65ec68:	vpxord xmm23,xmm23,xmm23
  65ec6e:	je     65e42d <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x105d>
  65ec74:	vmovdqa64 zmm23,ZMMWORD PTR [rip+0xffffffffff9b2dc2]        # 11a40 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x9f2>
  65ec7e:	vpxord xmm20,xmm20,xmm20
  65ec84:	jne    65e439 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1069>
  65ec8a:	vpxor  xmm14,xmm14,xmm14
  65ec8f:	je     65e44e <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x107e>
  65ec95:	vmovdqa64 zmm14,ZMMWORD PTR [rip+0xffffffffff9b2e21]        # 11ac0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xa72>
  65ec9f:	vpxor  xmm6,xmm6,xmm6
  65eca3:	jne    65e458 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1088>
  65eca9:	vpxord xmm31,xmm31,xmm31
  65ecaf:	je     65e46e <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x109e>
  65ecb5:	vmovdqa64 zmm31,ZMMWORD PTR [rip+0xffffffffff9b2e81]        # 11b40 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xaf2>
  65ecbf:	vpxord xmm28,xmm28,xmm28
  65ecc5:	jne    65e47a <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x10aa>
  65eccb:	vmovdqu64 ZMMWORD PTR [rsp+0x410],zmm1
  65ecd6:	vpxord xmm24,xmm24,xmm24
  65ecdc:	je     65e49b <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x10cb>
  65ece2:	vmovdqa64 zmm24,ZMMWORD PTR [rip+0xffffffffff9b2ed4]        # 11bc0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xb72>
  65ecec:	vpxor  xmm1,xmm1,xmm1
  65ecf0:	test   r15b,0x2
  65ecf4:	jne    65e4a9 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x10d9>
  65ecfa:	vpxor  xmm4,xmm4,xmm4
  65ecfe:	je     65e4bd <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x10ed>
  65ed04:	vmovdqa64 zmm4,ZMMWORD PTR [rip+0xffffffffff9b2f32]        # 11c40 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xbf2>
  65ed0e:	vpxord xmm30,xmm30,xmm30
  65ed14:	jne    65e4c9 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x10f9>
  65ed1a:	vpxord xmm25,xmm25,xmm25
  65ed20:	je     65e4df <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x110f>
  65ed26:	vmovdqa64 zmm25,ZMMWORD PTR [rip+0xffffffffff9b2f90]        # 11cc0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xc72>
  65ed30:	vpxord xmm21,xmm21,xmm21
  65ed36:	jne    65e4eb <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x111b>
  65ed3c:	vpxord xmm16,xmm16,xmm16
  65ed42:	je     65e501 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1131>
  65ed48:	vmovdqa64 zmm16,ZMMWORD PTR [rip+0xffffffffff9b2fee]        # 11d40 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xcf2>
  65ed52:	vxorps xmm8,xmm8,xmm8
  65ed57:	jne    65e50c <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x113c>
  65ed5d:	vmovups ZMMWORD PTR [rsp+0x1d0],zmm8
  65ed68:	je     65e527 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1157>
  65ed6e:	vmovaps zmm8,ZMMWORD PTR [rip+0xffffffffff9b3048]        # 11dc0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xd72>
  65ed78:	vmovups ZMMWORD PTR [rsp+0x50],zmm8
  65ed83:	vmovdqa64 zmm8,ZMMWORD PTR [rip+0xffffffffff9b3073]        # 11e00 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xdb2>
  65ed8d:	test   r15b,0x1
  65ed91:	jne    65e537 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1167>
  65ed97:	jmp    65e541 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1171>
  65ed9c:	mov    QWORD PTR [rsp+0x10],rbx
  65eda1:	lea    r15,[rsp+0x1590]
  65eda9:	xor    ebx,ebx
  65edab:	mov    edx,0x800
  65edb0:	mov    rdi,r15
  65edb3:	xor    esi,esi
  65edb5:	vzeroupper 
  65edb8:	call   QWORD PTR [rip+0x99482]        # 6f8240 <memset@GLIBC_2.2.5>
  65edbe:	vpxor  xmm11,xmm11,xmm11
  65edc3:	vpclmullqlqdq xmm16,xmm11,xmm11
  65edca:	vpshufd xmm1,xmm16,0xee
  65edd1:	mov    eax,0x87
  65edd6:	vmovq  xmm0,rax
  65eddb:	vpclmullqlqdq xmm17,xmm1,xmm0
  65ede2:	vpxorq xmm1,xmm1,xmm16
  65ede8:	vpclmullqlqdq xmm28,xmm1,xmm0
  65edef:	vmovq  rax,xmm28
  65edf5:	vpextrq rdx,xmm17,0x1
  65edfc:	lea    rcx,[rdx+rdx*1]
  65ee00:	lea    rsi,[rdx*4+0x0]
  65ee08:	xor    rsi,rax
  65ee0b:	mov    rax,rdx
  65ee0e:	shl    rax,0x7
  65ee12:	xor    rax,rcx
  65ee15:	xor    rax,rsi
  65ee18:	movabs rcx,0xdcb364640a222fe
  65ee22:	vmovq  xmm2,rcx
  65ee27:	vpclmullqlqdq xmm2,xmm11,xmm2
  65ee2d:	vpshufd xmm3,xmm2,0xee
  65ee32:	vpclmullqlqdq xmm13,xmm3,xmm0
  65ee38:	movabs rcx,0x3d5bd35c94646a24
  65ee42:	vmovq  xmm3,rcx
  65ee47:	vpclmullqlqdq xmm3,xmm11,xmm3
  65ee4d:	vpshufd xmm4,xmm3,0xee
  65ee52:	vpclmullqlqdq xmm14,xmm4,xmm0
  65ee58:	vpunpckhqdq xmm15,xmm13,xmm14
  65ee5d:	vpmovsxbq xmm10,WORD PTR [rip+0xffffffffff9b35da]        # 12440 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x13f2>
  65ee66:	vpmovsxbq xmm12,WORD PTR [rip+0xffffffffff9b35d3]        # 12442 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x13f4>
  65ee6f:	vpsllvq xmm4,xmm15,xmm10
  65ee74:	vpsllvq xmm5,xmm15,xmm12
  65ee79:	vpxorq xmm18,xmm5,xmm4
  65ee7f:	vpsllq xmm29,xmm15,0x7
  65ee86:	movabs rcx,0x6d58c4e181f9199f
  65ee90:	vmovq  xmm5,rcx
  65ee95:	vpclmullqlqdq xmm5,xmm11,xmm5
  65ee9b:	vpshufd xmm6,xmm5,0xee
  65eea0:	vpclmullqlqdq xmm19,xmm6,xmm0
  65eea7:	movabs rcx,0xa72ec17764d7ced5
  65eeb1:	vmovq  xmm6,rcx
  65eeb6:	vpclmullqlqdq xmm6,xmm11,xmm6
  65eebc:	vpshufd xmm7,xmm6,0xee
  65eec1:	vpclmullqlqdq xmm20,xmm7,xmm0
  65eec8:	vpunpckhqdq xmm21,xmm19,xmm20
  65eece:	vpsllvq xmm7,xmm21,xmm10
  65eed4:	vpsllvq xmm8,xmm21,xmm12
  65eeda:	vpxorq xmm22,xmm8,xmm7
  65eee0:	vpsllq xmm7,xmm21,0x7
  65eee7:	movabs rcx,0x4d52354a3a3d8c86
  65eef1:	vmovq  xmm8,rcx
  65eef6:	vpclmullqlqdq xmm8,xmm11,xmm8
  65eefc:	vpshufd xmm9,xmm8,0xee
  65ef02:	vpclmullqlqdq xmm23,xmm9,xmm0
  65ef09:	movabs rcx,0x553e92e8bc0ae9a7
  65ef13:	vmovq  xmm9,rcx
  65ef18:	vpclmullqlqdq xmm9,xmm11,xmm9
  65ef1e:	vpshufd xmm24,xmm9,0xee
  65ef25:	vpclmullqlqdq xmm24,xmm24,xmm0
  65ef2c:	vpunpckhqdq xmm25,xmm23,xmm24
  65ef32:	vpsllvq xmm10,xmm25,xmm10
  65ef38:	vpsllvq xmm12,xmm25,xmm12
  65ef3e:	vpxorq xmm26,xmm12,xmm10
  65ef44:	vpsllq xmm10,xmm25,0x7
  65ef4b:	movabs rcx,0x93252331bf042b11
  65ef55:	vmovq  xmm12,rcx
  65ef5a:	vpclmullqlqdq xmm11,xmm11,xmm12
  65ef60:	vpshufd xmm12,xmm11,0xee
  65ef66:	vpclmullqlqdq xmm12,xmm12,xmm0
  65ef6c:	vpextrq rsi,xmm12,0x1
  65ef72:	lea    rcx,[rsi+rsi*1]
  65ef76:	lea    rdi,[rsi*4+0x0]
  65ef7e:	xor    rdi,rcx
  65ef81:	mov    rcx,rsi
  65ef84:	shl    rcx,0x7
  65ef88:	vpternlogq xmm29,xmm15,xmm18,0x96
  65ef8f:	vpternlogq xmm7,xmm21,xmm22,0x96
  65ef96:	vpternlogq xmm10,xmm25,xmm26,0x96
  65ef9d:	vpunpcklqdq xmm13,xmm13,xmm14
  65efa2:	vpunpcklqdq xmm14,xmm19,xmm20
  65efa8:	vpunpcklqdq xmm15,xmm23,xmm24
  65efae:	xor    rax,rdx
  65efb1:	vpxorq xmm16,xmm17,xmm16
  65efb7:	xor    rcx,rsi
  65efba:	xor    rcx,rdi
  65efbd:	mov    edx,0x1
  65efc2:	vmovdqa64 ymm17,YMMWORD PTR [rip+0xffffffffff9ae614]        # d5e0 <anon.36d21461a6996b7f2bd9a76e3225918f.28.llvm.7983859039285366149+0x440>
  65efcc:	vpermt2q ymm29,ymm17,ymm16
  65efd2:	vmovdqa64 ymm16,YMMWORD PTR [rip+0xffffffffff9ae224]        # d200 <anon.36d21461a6996b7f2bd9a76e3225918f.28.llvm.7983859039285366149+0x60>
  65efdc:	movabs rsi,0x41a12db1f974f3ac
  65efe6:	vmovq  xmm17,rsi
  65efec:	movabs r10,0x2cf9e950788dea33
  65eff6:	movabs r11,0x5e2f716f4ede412f
  65f000:	movabs r9,0xf901b0182a098ffa
  65f00a:	vmovdqa64 ymm18,YMMWORD PTR [rip+0xffffffffff9ae48c]        # d4a0 <anon.36d21461a6996b7f2bd9a76e3225918f.28.llvm.7983859039285366149+0x300>
  65f014:	movabs r14,0x5cb10fbabcf00118
  65f01e:	movabs r13,0x95ed1f57f3632d4d
  65f028:	movabs rbp,0xc0d38dbf4f69c4ea
  65f032:	movabs rsi,0x11e33af086cd8d9e
  65f03c:	movabs rdi,0x512625b1f09fa87e
  65f046:	movabs r8,0xc20306804f9b836f
  65f050:	mov    r12d,ebx
  65f053:	shlx   r12,rdx,r12
  65f058:	vmovq  xmm19,r12
  65f05e:	vmovq  xmm20,rdx
  65f064:	vpclmullqlqdq xmm20,xmm19,xmm20
  65f06b:	vmovq  r12,xmm20
  65f071:	xor    r12,rax
  65f074:	mov    QWORD PTR [r15],r12
  65f077:	movabs r12,0x6b8330483c2e9849
  65f081:	vmovq  xmm21,r12
  65f087:	vpclmullqlqdq xmm21,xmm19,xmm21
  65f08e:	movabs r12,0x6648060e7c8cbab7
  65f098:	vmovq  xmm22,r12
  65f09e:	vpclmullqlqdq xmm22,xmm19,xmm22
  65f0a5:	vpternlogq xmm22,xmm21,xmm2,0x96
  65f0ac:	vpshufd xmm23,xmm22,0xee
  65f0b3:	vpxorq xmm23,xmm23,xmm2
  65f0b9:	vpclmullqlqdq xmm23,xmm23,xmm0
  65f0c0:	movabs r12,0x7573da4a5f7710ed
  65f0ca:	vmovq  xmm24,r12
  65f0d0:	vpclmullqlqdq xmm24,xmm19,xmm24
  65f0d7:	movabs r12,0x48280916cb137ac9
  65f0e1:	vmovq  xmm25,r12
  65f0e7:	vpclmullqlqdq xmm25,xmm19,xmm25
  65f0ee:	vpternlogq xmm25,xmm24,xmm3,0x96
  65f0f5:	vpshufd xmm26,xmm25,0xee
  65f0fc:	vpxorq xmm26,xmm26,xmm3
  65f102:	vpclmullqlqdq xmm26,xmm26,xmm0
  65f109:	vpunpckhqdq xmm27,xmm21,xmm24
  65f10f:	vpxorq xmm27,xmm13,xmm27
  65f115:	vpunpcklqdq xmm22,xmm22,xmm25
  65f11b:	vpunpckhqdq xmm25,xmm23,xmm26
  65f121:	vpternlogq xmm25,xmm27,xmm22,0x96
  65f128:	vinserti32x4 ymm1,ymm0,xmm25,0x1
  65f12f:	vpunpcklqdq xmm22,xmm23,xmm26
  65f135:	vinserti32x4 ymm20,ymm22,xmm20,0x1
  65f13c:	vpermq ymm20,ymm20,0x43
  65f143:	vpunpcklqdq xmm4,xmm21,xmm24
  65f149:	vpermt2q ymm4,ymm16,ymm28
  65f14f:	vpternlogq ymm4,ymm29,ymm20,0x96
  65f156:	vpblendd ymm1,ymm4,ymm1,0x30
  65f15c:	vmovdqu YMMWORD PTR [r15+0x8],ymm1
  65f162:	vpclmullqlqdq xmm1,xmm19,xmm17
  65f169:	vmovq  xmm4,r10
  65f16e:	vpclmullqlqdq xmm4,xmm19,xmm4
  65f175:	vpternlogq xmm4,xmm1,xmm5,0x96
  65f17c:	vpshufd xmm20,xmm4,0xee
  65f183:	vpxorq xmm20,xmm20,xmm5
  65f189:	vpclmullqlqdq xmm20,xmm20,xmm0
  65f190:	vmovq  xmm21,r11
  65f196:	vpclmullqlqdq xmm21,xmm19,xmm21
  65f19d:	vmovq  xmm22,r9
  65f1a3:	vpclmullqlqdq xmm22,xmm19,xmm22
  65f1aa:	vpternlogq xmm22,xmm21,xmm6,0x96
  65f1b1:	vpshufd xmm23,xmm22,0xee
  65f1b8:	vpxorq xmm23,xmm23,xmm6
  65f1be:	vpclmullqlqdq xmm23,xmm23,xmm0
  65f1c5:	vpunpcklqdq xmm24,xmm1,xmm21
  65f1cb:	vpunpcklqdq xmm26,xmm20,xmm23
  65f1d1:	vpternlogq xmm26,xmm7,xmm24,0x96
  65f1d8:	vinserti32x4 ymm24,ymm25,xmm26,0x1
  65f1df:	vpunpckhqdq xmm1,xmm1,xmm21
  65f1e5:	vpxor  xmm1,xmm14,xmm1
  65f1e9:	vpunpcklqdq xmm4,xmm4,xmm22
  65f1ef:	vpunpckhqdq xmm20,xmm20,xmm23
  65f1f5:	vpternlogq xmm20,xmm1,xmm4,0x96
  65f1fc:	vpermt2q ymm24,ymm18,ymm20
  65f202:	vmovdqu64 YMMWORD PTR [r15+0x28],ymm24
  65f20c:	vmovq  xmm4,r14
  65f211:	vpclmullqlqdq xmm4,xmm19,xmm4
  65f218:	vmovq  xmm20,r13
  65f21e:	vpclmullqlqdq xmm20,xmm19,xmm20
  65f225:	vpunpcklqdq xmm21,xmm4,xmm20
  65f22b:	vinserti32x4 ymm1,ymm1,xmm21,0x1
  65f232:	vmovq  xmm21,rbp
  65f238:	vpclmullqlqdq xmm21,xmm19,xmm21
  65f23f:	vpternlogq xmm21,xmm20,xmm9,0x96
  65f246:	vpshufd xmm24,xmm21,0xee
  65f24d:	vpxorq xmm24,xmm24,xmm9
  65f253:	vpclmullqlqdq xmm24,xmm24,xmm0
  65f25a:	vinserti32x4 ymm22,ymm22,xmm24,0x1
  65f261:	vmovq  xmm25,rsi
  65f267:	vpclmullqlqdq xmm25,xmm19,xmm25
  65f26e:	vpternlogq xmm25,xmm4,xmm8,0x96
  65f275:	vpshufd xmm26,xmm25,0xee
  65f27c:	vpxorq xmm26,xmm26,xmm8
  65f282:	vpclmullqlqdq xmm26,xmm26,xmm0
  65f289:	vinserti32x4 ymm27,ymm0,xmm26,0x1
  65f290:	vpunpcklqdq ymm22,ymm27,ymm22
  65f296:	vinserti32x4 ymm23,ymm23,xmm10,0x1
  65f29d:	vpternlogq ymm23,ymm1,ymm22,0x96
  65f2a4:	vpunpckhqdq xmm1,xmm4,xmm20
  65f2aa:	vpxor  xmm1,xmm15,xmm1
  65f2ae:	vpunpcklqdq xmm4,xmm25,xmm21
  65f2b4:	vpunpckhqdq xmm20,xmm26,xmm24
  65f2ba:	vpternlogq xmm20,xmm1,xmm4,0x96
  65f2c1:	vpermt2q ymm23,ymm18,ymm20
  65f2c7:	vmovdqu64 YMMWORD PTR [r15+0x48],ymm23
  65f2d1:	vpextrq QWORD PTR [r15+0x68],xmm20,0x1
  65f2d9:	vmovq  xmm1,rdi
  65f2de:	vpclmullqlqdq xmm1,xmm19,xmm1
  65f2e5:	vmovq  xmm4,r8
  65f2ea:	vpclmullqlqdq xmm4,xmm19,xmm4
  65f2f1:	vpternlogq xmm4,xmm1,xmm11,0x96
  65f2f8:	vpshufd xmm19,xmm4,0xee
  65f2ff:	vpxorq xmm19,xmm19,xmm11
  65f305:	vpclmullqlqdq xmm19,xmm19,xmm0
  65f30c:	vpxorq xmm1,xmm1,xmm19
  65f312:	vmovq  r12,xmm1
  65f317:	xor    r12,rcx
  65f31a:	mov    QWORD PTR [r15+0x70],r12
  65f31e:	inc    rbx
  65f321:	vpshufd xmm1,xmm1,0xee
  65f326:	vpternlogq xmm1,xmm4,xmm12,0x96
  65f32d:	vmovq  QWORD PTR [r15+0x78],xmm1
  65f333:	sub    r15,0xffffffffffffff80
  65f337:	cmp    rbx,0x10
  65f33b:	jne    65f050 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1c80>
  65f341:	lea    rdi,[rsp+0xd90]
  65f349:	xor    r14d,r14d
  65f34c:	mov    rbx,QWORD PTR [rip+0x98eed]        # 6f8240 <memset@GLIBC_2.2.5>
  65f353:	mov    edx,0x800
  65f358:	xor    esi,esi
  65f35a:	vzeroupper 
  65f35d:	call   rbx
  65f35f:	lea    rdi,[rsp+0x590]
  65f367:	mov    edx,0x800
  65f36c:	xor    esi,esi
  65f36e:	call   rbx
  65f370:	vpmovsxbq xmm0,WORD PTR [rip+0xffffffffff9b30cb]        # 12444 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x13f6>
  65f379:	xor    eax,eax
  65f37b:	nop    DWORD PTR [rax+rax*1+0x0]
  65f380:	xor    edx,edx
  65f382:	mov    ecx,eax
  65f384:	mov    esi,0x1
  65f389:	shld   rdx,rsi,cl
  65f38d:	mov    rcx,rax
  65f390:	shl    rcx,0x4
  65f394:	shlx   rsi,rsi,rax
  65f399:	test   al,0x40
  65f39b:	cmovne rdx,rsi
  65f39f:	cmovne rsi,r14
  65f3a3:	mov    QWORD PTR [rsp+rcx*1+0x598],rdx
  65f3ab:	mov    QWORD PTR [rsp+rcx*1+0x590],rsi
  65f3b3:	lea    rdx,[rsp+rcx*1+0xd90]
  65f3bb:	mov    QWORD PTR [rsp+0x90],rdx
  65f3c3:	mov    rsi,QWORD PTR [rsp+rcx*1+0xd90]
  65f3cb:	mov    rcx,QWORD PTR [rsp+rcx*1+0xd98]
  65f3d3:	xor    r13d,r13d
  65f3d6:	vmovdqa xmm1,xmm0
  65f3da:	xor    r15d,r15d
  65f3dd:	xor    r9d,r9d
  65f3e0:	xor    r12d,r12d
  65f3e3:	xor    r8d,r8d
  65f3e6:	xor    edi,edi
  65f3e8:	xor    r10d,r10d
  65f3eb:	nop    DWORD PTR [rax+rax*1+0x0]
  65f3f0:	mov    QWORD PTR [rsp+0xd0],r10
  65f3f8:	mov    QWORD PTR [rsp+0x110],r8
  65f400:	mov    QWORD PTR [rsp+0x150],rdi
  65f408:	mov    QWORD PTR [rsp+0x190],rsi
  65f410:	mov    QWORD PTR [rsp+0x50],rcx
  65f415:	mov    rdx,QWORD PTR [rsp+r13*1+0x15a8]
  65f41d:	mov    rsi,QWORD PTR [rsp+r13*1+0x15a0]
  65f425:	mov    ecx,eax
  65f427:	shrd   rsi,rdx,cl
  65f42b:	mov    rdi,QWORD PTR [rsp+r13*1+0x1590]
  65f433:	mov    r8,QWORD PTR [rsp+r13*1+0x1598]
  65f43b:	shrd   rdi,r8,cl
  65f43f:	mov    r11,QWORD PTR [rsp+r13*1+0x15c8]
  65f447:	mov    rbx,QWORD PTR [rsp+r13*1+0x15c0]
  65f44f:	shrd   rbx,r11,cl
  65f453:	mov    rbp,QWORD PTR [rsp+r13*1+0x15b8]
  65f45b:	mov    r14,QWORD PTR [rsp+r13*1+0x15b0]
  65f463:	shrd   r14,rbp,cl
  65f467:	shrx   r8,r8,rax
  65f46c:	test   al,0x40
  65f46e:	cmove  r8,rdi
  65f472:	shrx   rdi,rdx,rax
  65f477:	cmove  rdi,rsi
  65f47b:	shrx   rdx,rbp,rax
  65f480:	cmove  rdx,r14
  65f484:	shrx   rbp,r11,rax
  65f489:	cmove  rbp,rbx
  65f48d:	and    edi,0x1
  65f490:	and    r8d,0x1
  65f494:	and    ebp,0x1
  65f497:	vmovd  ecx,xmm1
  65f49b:	xor    esi,esi
  65f49d:	shld   rsi,r8,cl
  65f4a1:	and    edx,0x1
  65f4a4:	shlx   rbx,r8,rcx
  65f4a9:	test   cl,0x40
  65f4ac:	cmovne rsi,rbx
  65f4b0:	vpextrb ecx,xmm1,0x8
  65f4b6:	mov    r10d,0x0
  65f4bc:	cmovne rbx,r10
  65f4c0:	xor    r8d,r8d
  65f4c3:	shld   r8,rdi,cl
  65f4c7:	vpaddq xmm2,xmm1,QWORD PTR [rip+0xffffffffff9ad807]{1to2}        # ccd8 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0x58>
  65f4d1:	shlx   rdi,rdi,rcx
  65f4d6:	test   cl,0x40
  65f4d9:	cmovne r8,rdi
  65f4dd:	cmovne rdi,r10
  65f4e1:	vmovd  ecx,xmm2
  65f4e5:	xor    r11d,r11d
  65f4e8:	shld   r11,rdx,cl
  65f4ec:	shlx   rdx,rdx,rcx
  65f4f1:	test   cl,0x40
  65f4f4:	cmovne r11,rdx
  65f4f8:	vpextrb ecx,xmm2,0x8
  65f4fe:	cmovne rdx,r10
  65f502:	xor    r14d,r14d
  65f505:	shld   r14,rbp,cl
  65f509:	shlx   rbp,rbp,rcx
  65f50e:	test   cl,0x40
  65f511:	mov    rcx,QWORD PTR [rsp+0x50]
  65f516:	cmovne r14,rbp
  65f51a:	cmovne rbp,r10
  65f51e:	mov    r10,QWORD PTR [rsp+0xd0]
  65f526:	or     r9,r8
  65f529:	mov    r8,QWORD PTR [rsp+0x110]
  65f531:	or     rcx,rsi
  65f534:	mov    rsi,QWORD PTR [rsp+0x190]
  65f53c:	or     r15,rdi
  65f53f:	mov    rdi,QWORD PTR [rsp+0x150]
  65f547:	or     rsi,rbx
  65f54a:	or     r10,r14
  65f54d:	or     r8,r11
  65f550:	or     rdi,rbp
  65f553:	or     r12,rdx
  65f556:	vpaddq xmm1,xmm1,QWORD PTR [rip+0xffffffffff9ad7d8]{1to2}        # cd38 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0xb8>
  65f560:	add    r13,0x40
  65f564:	cmp    r13,0x800
  65f56b:	jne    65f3f0 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x2020>
  65f571:	or     r12,rsi
  65f574:	or     rdi,r15
  65f577:	or     rdi,r12
  65f57a:	or     r8,rcx
  65f57d:	or     r10,r9
  65f580:	or     r10,r8
  65f583:	inc    rax
  65f586:	vmovq  xmm1,rdi
  65f58b:	vmovq  xmm2,r10
  65f590:	vpunpcklqdq xmm1,xmm1,xmm2
  65f594:	mov    rcx,QWORD PTR [rsp+0x90]
  65f59c:	vmovdqa XMMWORD PTR [rcx],xmm1
  65f5a0:	cmp    rax,0x80
  65f5a6:	mov    r14d,0x0
  65f5ac:	jne    65f380 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x1fb0>
  65f5b2:	mov    rax,0xfffffffffffffff0
  65f5b9:	xor    edi,edi
  65f5bb:	xor    ecx,ecx
  65f5bd:	jmp    65f5da <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x220a>
  65f5bf:	nop
  65f5c0:	add    rax,0x10
  65f5c4:	add    rdi,0x10
  65f5c8:	mov    rcx,QWORD PTR [rsp+0x50]
  65f5cd:	cmp    rcx,0x80
  65f5d4:	je     65f78d <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x23bd>
  65f5da:	lea    rsi,[rcx+0x1]
  65f5de:	mov    rdx,rax
  65f5e1:	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  65f5f0:	lea    r9,[rdx+0x10]
  65f5f4:	cmp    r9,0x800
  65f5fb:	je     6605e2 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x3212>
  65f601:	mov    r10,QWORD PTR [rsp+rdx*1+0xda0]
  65f609:	mov    rdx,QWORD PTR [rsp+rdx*1+0xda8]
  65f611:	shrd   r10,rdx,cl
  65f615:	shrx   r11,rdx,rcx
  65f61a:	test   cl,0x40
  65f61d:	cmove  r11,r10
  65f621:	mov    rdx,r9
  65f624:	test   r11b,0x1
  65f628:	je     65f5f0 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x2220>
  65f62a:	mov    QWORD PTR [rsp+0x50],rsi
  65f62f:	mov    rdx,rcx
  65f632:	shl    rdx,0x4
  65f636:	vmovaps xmm0,XMMWORD PTR [rsp+rdx*1+0xd90]
  65f63f:	vmovaps xmm1,XMMWORD PTR [rsp+r9*1+0xd90]
  65f649:	vmovaps XMMWORD PTR [rsp+rdx*1+0xd90],xmm1
  65f652:	vmovaps XMMWORD PTR [rsp+r9*1+0xd90],xmm0
  65f65c:	vmovdqa xmm0,XMMWORD PTR [rsp+rdx*1+0x590]
  65f665:	vmovdqa xmm1,XMMWORD PTR [rsp+r9*1+0x590]
  65f66f:	vmovdqa XMMWORD PTR [rsp+rdx*1+0x590],xmm1
  65f678:	vmovdqa XMMWORD PTR [rsp+r9*1+0x590],xmm0
  65f682:	mov    r9,QWORD PTR [rsp+rdx*1+0xd90]
  65f68a:	mov    r10,QWORD PTR [rsp+rdx*1+0xd98]
  65f692:	mov    r11,QWORD PTR [rsp+rdx*1+0x590]
  65f69a:	mov    rbx,QWORD PTR [rsp+rdx*1+0x598]
  65f6a2:	xor    r14d,r14d
  65f6a5:	mov    edx,0x1
  65f6aa:	shld   r14,rdx,cl
  65f6ae:	shlx   r15,rdx,rcx
  65f6b3:	test   cl,0x40
  65f6b6:	cmovne r14,r15
  65f6ba:	mov    edx,0x0
  65f6bf:	cmovne r15,rdx
  65f6c3:	xor    r12d,r12d
  65f6c6:	xor    r13d,r13d
  65f6c9:	jmp    65f6e5 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x2315>
  65f6cb:	nop    DWORD PTR [rax+rax*1+0x0]
  65f6d0:	add    r13,0x2
  65f6d4:	add    r12,0x20
  65f6d8:	cmp    r13,0x80
  65f6df:	je     65f5c0 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x21f0>
  65f6e5:	cmp    rdi,r12
  65f6e8:	je     65f731 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x2361>
  65f6ea:	mov    rdx,QWORD PTR [rsp+r12*1+0xd90]
  65f6f2:	mov    rbp,QWORD PTR [rsp+r12*1+0xd98]
  65f6fa:	mov    rsi,rbp
  65f6fd:	and    rsi,r14
  65f700:	mov    r8,rdx
  65f703:	and    r8,r15
  65f706:	or     r8,rsi
  65f709:	je     65f731 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x2361>
  65f70b:	xor    rbp,r10
  65f70e:	xor    rdx,r9
  65f711:	mov    QWORD PTR [rsp+r12*1+0xd90],rdx
  65f719:	mov    QWORD PTR [rsp+r12*1+0xd98],rbp
  65f721:	xor    QWORD PTR [rsp+r12*1+0x590],r11
  65f729:	xor    QWORD PTR [rsp+r12*1+0x598],rbx
  65f731:	mov    rdx,r13
  65f734:	or     rdx,0x1
  65f738:	cmp    rdx,rcx
  65f73b:	je     65f6d0 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x2300>
  65f73d:	mov    r8,QWORD PTR [rsp+r12*1+0xda0]
  65f745:	mov    rsi,QWORD PTR [rsp+r12*1+0xda8]
  65f74d:	mov    rdx,rsi
  65f750:	and    rdx,r14
  65f753:	mov    rbp,r8
  65f756:	and    rbp,r15
  65f759:	or     rbp,rdx
  65f75c:	je     65f6d0 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x2300>
  65f762:	xor    rsi,r10
  65f765:	xor    r8,r9
  65f768:	mov    QWORD PTR [rsp+r12*1+0xda0],r8
  65f770:	mov    QWORD PTR [rsp+r12*1+0xda8],rsi
  65f778:	xor    QWORD PTR [rsp+r12*1+0x5a0],r11
  65f780:	xor    QWORD PTR [rsp+r12*1+0x5a8],rbx
  65f788:	jmp    65f6d0 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x2300>
  65f78d:	lea    rdi,[rsp+0x1d90]
  65f795:	mov    edx,0x800
  65f79a:	xor    esi,esi
  65f79c:	call   QWORD PTR [rip+0x98a9e]        # 6f8240 <memset@GLIBC_2.2.5>
  65f7a2:	vpbroadcastq xmm4,QWORD PTR [rip+0xffffffffff9ad58d]        # cd38 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0xb8>
  65f7ab:	vpbroadcastq xmm3,QWORD PTR [rip+0xffffffffff9ad524]        # ccd8 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0x58>
  65f7b4:	vpmovsxbq xmm0,WORD PTR [rip+0xffffffffff9b2c87]        # 12444 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x13f6>
  65f7bd:	xor    eax,eax
  65f7bf:	xor    ecx,ecx
  65f7c1:	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  65f7d0:	mov    rdx,QWORD PTR [rsp+rcx*1+0x1d90]
  65f7d8:	mov    QWORD PTR [rsp+0x90],rcx
  65f7e0:	mov    rcx,QWORD PTR [rsp+rcx*1+0x1d98]
  65f7e8:	xor    r12d,r12d
  65f7eb:	vmovdqa xmm1,xmm0
  65f7ef:	xor    r14d,r14d
  65f7f2:	xor    r9d,r9d
  65f7f5:	xor    r15d,r15d
  65f7f8:	xor    r8d,r8d
  65f7fb:	xor    esi,esi
  65f7fd:	xor    edi,edi
  65f7ff:	nop
  65f800:	mov    QWORD PTR [rsp+0xd0],r8
  65f808:	mov    QWORD PTR [rsp+0x110],rdi
  65f810:	mov    QWORD PTR [rsp+0x150],rsi
  65f818:	mov    QWORD PTR [rsp+0x190],rdx
  65f820:	mov    QWORD PTR [rsp+0x50],rcx
  65f825:	mov    rdx,QWORD PTR [rsp+r12*1+0x5a8]
  65f82d:	mov    r10,QWORD PTR [rsp+r12*1+0x5a0]
  65f835:	mov    ecx,eax
  65f837:	shrd   r10,rdx,cl
  65f83b:	mov    rsi,QWORD PTR [rsp+r12*1+0x590]
  65f843:	mov    rdi,QWORD PTR [rsp+r12*1+0x598]
  65f84b:	shrd   rsi,rdi,cl
  65f84f:	mov    rbx,QWORD PTR [rsp+r12*1+0x5c8]
  65f857:	mov    r11,QWORD PTR [rsp+r12*1+0x5c0]
  65f85f:	shrd   r11,rbx,cl
  65f863:	mov    r13,QWORD PTR [rsp+r12*1+0x5b8]
  65f86b:	mov    r8,QWORD PTR [rsp+r12*1+0x5b0]
  65f873:	shrd   r8,r13,cl
  65f877:	shrx   rdi,rdi,rax
  65f87c:	test   al,0x40
  65f87e:	cmove  rdi,rsi
  65f882:	shrx   rsi,rdx,rax
  65f887:	cmove  rsi,r10
  65f88b:	shrx   rbp,r13,rax
  65f890:	cmove  rbp,r8
  65f894:	shrx   r13,rbx,rax
  65f899:	cmove  r13,r11
  65f89d:	and    esi,0x1
  65f8a0:	and    edi,0x1
  65f8a3:	and    r13d,0x1
  65f8a7:	vmovd  ecx,xmm1
  65f8ab:	xor    edx,edx
  65f8ad:	shld   rdx,rdi,cl
  65f8b1:	and    ebp,0x1
  65f8b4:	shlx   rbx,rdi,rcx
  65f8b9:	test   cl,0x40
  65f8bc:	cmovne rdx,rbx
  65f8c0:	vpextrb ecx,xmm1,0x8
  65f8c6:	mov    edi,0x0
  65f8cb:	cmovne rbx,rdi
  65f8cf:	xor    edi,edi
  65f8d1:	shld   rdi,rsi,cl
  65f8d5:	vpaddq xmm2,xmm1,xmm3
  65f8d9:	shlx   rsi,rsi,rcx
  65f8de:	test   cl,0x40
  65f8e1:	cmovne rdi,rsi
  65f8e5:	mov    ecx,0x0
  65f8ea:	cmovne rsi,rcx
  65f8ee:	vmovd  ecx,xmm2
  65f8f2:	xor    r10d,r10d
  65f8f5:	shld   r10,rbp,cl
  65f8f9:	shlx   rbp,rbp,rcx
  65f8fe:	test   cl,0x40
  65f901:	cmovne r10,rbp
  65f905:	vpextrb ecx,xmm2,0x8
  65f90b:	mov    r8d,0x0
  65f911:	cmovne rbp,r8
  65f915:	xor    r8d,r8d
  65f918:	shld   r8,r13,cl
  65f91c:	shlx   r11,r13,rcx
  65f921:	xor    r13d,r13d
  65f924:	test   cl,0x40
  65f927:	mov    rcx,QWORD PTR [rsp+0x50]
  65f92c:	cmovne r8,r11
  65f930:	cmovne r11,r13
  65f934:	or     r9,rdi
  65f937:	mov    rdi,QWORD PTR [rsp+0x110]
  65f93f:	or     rcx,rdx
  65f942:	mov    rdx,QWORD PTR [rsp+0x190]
  65f94a:	or     r14,rsi
  65f94d:	mov    rsi,QWORD PTR [rsp+0x150]
  65f955:	or     rdx,rbx
  65f958:	or     rdi,r8
  65f95b:	mov    r8,QWORD PTR [rsp+0xd0]
  65f963:	or     r8,r10
  65f966:	or     rsi,r11
  65f969:	or     r15,rbp
  65f96c:	vpaddq xmm1,xmm1,xmm4
  65f970:	add    r12,0x40
  65f974:	cmp    r12,0x800
  65f97b:	jne    65f800 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x2430>
  65f981:	or     r15,rdx
  65f984:	or     rsi,r14
  65f987:	or     rsi,r15
  65f98a:	or     r8,rcx
  65f98d:	or     rdi,r9
  65f990:	or     rdi,r8
  65f993:	inc    rax
  65f996:	vmovq  xmm1,rsi
  65f99b:	vmovq  xmm2,rdi
  65f9a0:	vpunpcklqdq xmm1,xmm1,xmm2
  65f9a4:	mov    rcx,QWORD PTR [rsp+0x90]
  65f9ac:	vmovdqa XMMWORD PTR [rsp+rcx*1+0x1d90],xmm1
  65f9b5:	add    rcx,0x10
  65f9b9:	cmp    rcx,0x800
  65f9c0:	jne    65f7d0 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x2400>
  65f9c6:	lea    rdi,[rsp+0x1d90]
  65f9ce:	call   670780 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5field11byte_tables>
  65f9d3:	mov    QWORD PTR [rsp],rax
  65f9d7:	add    rax,0x10
  65f9db:	mov    QWORD PTR [rsp+0x18],rax
  65f9e0:	xor    ecx,ecx
  65f9e2:	vpbroadcastd xmm0,DWORD PTR [rip+0xffffffffff9acc4d]        # c638 <anon.0fac96d756d24b1a23b798945b2a13ad.383.llvm.5929972809416623235+0x128>
  65f9eb:	vmovdqa xmm9,XMMWORD PTR [rip+0xffffffffff9acebd]        # c8b0 <anon.60695053c5df2db48177e894d4d8ab59.354.llvm.16546105846116086659+0xf0>
  65f9f3:	vpbroadcastq xmm2,QWORD PTR [rip+0xffffffffff9ad2cc]        # ccc8 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0x48>
  65f9fc:	vmovdqa xmm11,XMMWORD PTR [rip+0xffffffffff9ad06c]        # ca70 <anon.0e5291c933a25680de2bf032d375ada2.27.llvm.16073103944965275176+0xc0>
  65fa04:	vpbroadcastq xmm4,QWORD PTR [rip+0xffffffffff9ad2a3]        # ccb0 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0x30>
  65fa0d:	vmovdqa xmm13,XMMWORD PTR [rip+0xffffffffff9acf3b]        # c950 <anon.60695053c5df2db48177e894d4d8ab59.354.llvm.16546105846116086659+0x190>
  65fa15:	vpbroadcastq xmm6,QWORD PTR [rip+0xffffffffff9ad2fa]        # cd18 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0x98>
  65fa1e:	vmovdqa xmm14,XMMWORD PTR [rip+0xffffffffff9ad06a]        # ca90 <anon.0e5291c933a25680de2bf032d375ada2.27.llvm.16073103944965275176+0xe0>
  65fa26:	vpbroadcastq xmm8,QWORD PTR [rip+0xffffffffff9ad1d9]        # cc08 <anon.36d21461a6996b7f2bd9a76e3225918f.29.llvm.7983859039285366149+0xd8>
  65fa2f:	vmovdqa64 xmm17,XMMWORD PTR [rip+0xffffffffff9ad067]        # caa0 <anon.0e5291c933a25680de2bf032d375ada2.27.llvm.16073103944965275176+0xf0>
  65fa39:	vpbroadcastq xmm10,QWORD PTR [rip+0xffffffffff9ad256]        # cc98 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0x18>
  65fa42:	vmovdqa64 xmm18,XMMWORD PTR [rip+0xffffffffff9ad0f4]        # cb40 <anon.36d21461a6996b7f2bd9a76e3225918f.29.llvm.7983859039285366149+0x10>
  65fa4c:	vpbroadcastq xmm12,QWORD PTR [rip+0xffffffffff9ad28b]        # cce0 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0x60>
  65fa55:	vpbroadcastq xmm19,QWORD PTR [rip+0xffffffffff9ad249]        # cca8 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0x28>
  65fa5f:	vpbroadcastd xmm20,DWORD PTR [rip+0xffffffffff9acc83]        # c6ec <anon.0fac96d756d24b1a23b798945b2a13ad.103.llvm.5929972809416623235+0xa0>
  65fa69:	vpbroadcastd xmm15,DWORD PTR [rip+0xffffffffff9acbee]        # c660 <anon.0fac96d756d24b1a23b798945b2a13ad.103.llvm.5929972809416623235+0x14>
  65fa72:	vpbroadcastd xmm30,DWORD PTR [rip+0xffffffffff9acc84]        # c700 <anon.0fac96d756d24b1a23b798945b2a13ad.103.llvm.5929972809416623235+0xb4>
  65fa7c:	vpbroadcastd xmm16,DWORD PTR [rip+0xffffffffff9acc4e]        # c6d4 <anon.0fac96d756d24b1a23b798945b2a13ad.103.llvm.5929972809416623235+0x88>
  65fa86:	cs nop WORD PTR [rax+rax*1+0x0]
  65fa90:	mov    QWORD PTR [rsp+0x20],rcx
  65fa95:	lea    rcx,[rcx*8+0x0]
  65fa9d:	mov    rsi,QWORD PTR [rsp+0x18]
  65faa2:	xor    eax,eax
  65faa4:	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  65fab0:	mov    QWORD PTR [rsp+0x50],rax
  65fab5:	mov    r9,QWORD PTR [rsi]
  65fab8:	mov    rbx,QWORD PTR [rsi+0x1008]
  65fabf:	mov    r8,QWORD PTR [rsi+0x1000]
  65fac6:	shrd   r8,rbx,cl
  65faca:	mov    rax,QWORD PTR [rsi+0x8]
  65face:	shrd   r9,rax,cl
  65fad2:	shrx   rdi,rax,rcx
  65fad7:	mov    r13,QWORD PTR [rsi+0x1018]
  65fade:	mov    r11,QWORD PTR [rsi+0x1010]
  65fae5:	shrd   r11,r13,cl
  65fae9:	mov    r12,QWORD PTR [rsi+0x18]
  65faed:	mov    r10,QWORD PTR [rsi+0x10]
  65faf1:	shrd   r10,r12,cl
  65faf5:	mov    rax,QWORD PTR [rsi+0x1038]
  65fafc:	mov    QWORD PTR [rsp+0x30],rax
  65fb01:	mov    r15,QWORD PTR [rsi+0x1030]
  65fb08:	shrd   r15,rax,cl
  65fb0c:	mov    rax,QWORD PTR [rsi+0x38]
  65fb10:	mov    QWORD PTR [rsp+0x28],rax
  65fb15:	mov    r14,QWORD PTR [rsi+0x30]
  65fb19:	shrd   r14,rax,cl
  65fb1d:	mov    rdx,QWORD PTR [rsi+0x1078]
  65fb24:	mov    QWORD PTR [rsp+0x40],rdx
  65fb29:	mov    rax,QWORD PTR [rsi+0x1070]
  65fb30:	shrd   rax,rdx,cl
  65fb34:	mov    QWORD PTR [rsp+0x48],rax
  65fb39:	mov    rax,QWORD PTR [rsi+0x78]
  65fb3d:	mov    QWORD PTR [rsp+0x38],rax
  65fb42:	mov    rbp,QWORD PTR [rsi+0x70]
  65fb46:	shrd   rbp,rax,cl
  65fb4a:	mov    rdx,QWORD PTR [rsi+0x10f8]
  65fb51:	mov    QWORD PTR [rsp+0x250],rdx
  65fb59:	mov    rax,QWORD PTR [rsi+0x10f0]
  65fb60:	shrd   rax,rdx,cl
  65fb64:	mov    QWORD PTR [rsp+0x290],rax
  65fb6c:	mov    rdx,QWORD PTR [rsi+0xf8]
  65fb73:	mov    QWORD PTR [rsp+0x1d0],rdx
  65fb7b:	mov    rax,QWORD PTR [rsi+0xf0]
  65fb82:	shrd   rax,rdx,cl
  65fb86:	mov    QWORD PTR [rsp+0x210],rax
  65fb8e:	mov    rdx,QWORD PTR [rsi+0x11f8]
  65fb95:	mov    QWORD PTR [rsp+0x350],rdx
  65fb9d:	mov    rax,QWORD PTR [rsi+0x11f0]
  65fba4:	shrd   rax,rdx,cl
  65fba8:	mov    QWORD PTR [rsp+0x390],rax
  65fbb0:	mov    rdx,QWORD PTR [rsi+0x1f8]
  65fbb7:	mov    QWORD PTR [rsp+0x2d0],rdx
  65fbbf:	mov    rax,QWORD PTR [rsi+0x1f0]
  65fbc6:	shrd   rax,rdx,cl
  65fbca:	mov    QWORD PTR [rsp+0x310],rax
  65fbd2:	mov    rdx,QWORD PTR [rsi+0x13f8]
  65fbd9:	mov    QWORD PTR [rsp+0x450],rdx
  65fbe1:	mov    rax,QWORD PTR [rsi+0x13f0]
  65fbe8:	shrd   rax,rdx,cl
  65fbec:	mov    QWORD PTR [rsp+0x90],rax
  65fbf4:	mov    rdx,QWORD PTR [rsi+0x3f8]
  65fbfb:	mov    QWORD PTR [rsp+0x3d0],rdx
  65fc03:	mov    rax,QWORD PTR [rsi+0x3f0]
  65fc0a:	shrd   rax,rdx,cl
  65fc0e:	mov    QWORD PTR [rsp+0x410],rax
  65fc16:	mov    rdx,QWORD PTR [rsi+0x17f8]
  65fc1d:	mov    QWORD PTR [rsp+0x150],rdx
  65fc25:	mov    rax,QWORD PTR [rsi+0x17f0]
  65fc2c:	shrd   rax,rdx,cl
  65fc30:	mov    QWORD PTR [rsp+0x190],rax
  65fc38:	mov    rdx,QWORD PTR [rsi+0x7f8]
  65fc3f:	mov    QWORD PTR [rsp+0xd0],rdx
  65fc47:	mov    rax,QWORD PTR [rsi+0x7f0]
  65fc4e:	shrd   rax,rdx,cl
  65fc52:	mov    QWORD PTR [rsp+0x110],rax
  65fc5a:	test   cl,0x40
  65fc5d:	cmove  rdi,r9
  65fc61:	shrx   rdx,rbx,rcx
  65fc66:	cmove  rdx,r8
  65fc6a:	vmovq  xmm24,rdx
  65fc70:	vmovq  xmm25,rdi
  65fc76:	shrx   rax,r12,rcx
  65fc7b:	cmove  rax,r10
  65fc7f:	shrx   rdx,r13,rcx
  65fc84:	cmove  rdx,r11
  65fc88:	vmovq  xmm28,rdx
  65fc8e:	vmovq  xmm29,rax
  65fc94:	shrx   rax,QWORD PTR [rsp+0x28],rcx
  65fc9b:	cmove  rax,r14
  65fc9f:	shrx   rdx,QWORD PTR [rsp+0x30],rcx
  65fca6:	cmove  rdx,r15
  65fcaa:	vmovq  xmm1,rdx
  65fcaf:	vmovq  xmm3,rax
  65fcb4:	shrx   rax,QWORD PTR [rsp+0x38],rcx
  65fcbb:	cmove  rax,rbp
  65fcbf:	shrx   rdx,QWORD PTR [rsp+0x40],rcx
  65fcc6:	cmove  rdx,QWORD PTR [rsp+0x48]
  65fccc:	vmovq  xmm22,rdx
  65fcd2:	vmovq  xmm23,rax
  65fcd8:	shrx   rax,QWORD PTR [rsp+0x1d0],rcx
  65fce2:	cmove  rax,QWORD PTR [rsp+0x210]
  65fceb:	shrx   rdx,QWORD PTR [rsp+0x250],rcx
  65fcf5:	cmove  rdx,QWORD PTR [rsp+0x290]
  65fcfe:	vmovq  xmm26,rdx
  65fd04:	vmovq  xmm27,rax
  65fd0a:	shrx   rax,QWORD PTR [rsp+0x2d0],rcx
  65fd14:	cmove  rax,QWORD PTR [rsp+0x310]
  65fd1d:	shrx   rdx,QWORD PTR [rsp+0x350],rcx
  65fd27:	cmove  rdx,QWORD PTR [rsp+0x390]
  65fd30:	vmovq  xmm21,rdx
  65fd36:	vmovq  xmm31,rax
  65fd3c:	shrx   rax,QWORD PTR [rsp+0x3d0],rcx
  65fd46:	cmove  rax,QWORD PTR [rsp+0x410]
  65fd4f:	shrx   rdx,QWORD PTR [rsp+0x450],rcx
  65fd59:	cmove  rdx,QWORD PTR [rsp+0x90]
  65fd62:	vpunpcklbw xmm5,xmm25,xmm24
  65fd68:	vpand  xmm7,xmm5,xmm0
  65fd6c:	vpshufb xmm7,xmm7,xmm9
  65fd71:	vgf2p8affineqb xmm9,xmm5,xmm2,0x0
  65fd77:	vpand  xmm9,xmm9,xmm0
  65fd7b:	vpshufb xmm9,xmm9,xmm11
  65fd80:	vpor   xmm7,xmm9,xmm7
  65fd84:	vgf2p8affineqb xmm9,xmm5,xmm4,0x0
  65fd8a:	vpand  xmm9,xmm9,xmm0
  65fd8e:	vpshufb xmm9,xmm9,xmm13
  65fd93:	vgf2p8affineqb xmm11,xmm5,xmm6,0x0
  65fd99:	vpand  xmm11,xmm11,xmm0
  65fd9d:	vpshufb xmm11,xmm11,xmm14
  65fda2:	vpternlogq xmm11,xmm7,xmm9,0xfe
  65fda9:	vgf2p8affineqb xmm7,xmm5,xmm8,0x0
  65fdaf:	vpand  xmm7,xmm7,xmm0
  65fdb3:	vpshufb xmm9,xmm7,xmm17
  65fdb9:	vgf2p8affineqb xmm7,xmm5,xmm10,0x0
  65fdbf:	vpand  xmm7,xmm7,xmm0
  65fdc3:	vpshufb xmm7,xmm7,xmm18
  65fdc9:	vpternlogq xmm7,xmm11,xmm9,0xfe
  65fdd0:	vgf2p8affineqb xmm9,xmm5,xmm19,0x0
  65fdd7:	vpmovzxbq xmm9,xmm9
  65fddc:	vpunpcklbw xmm28,xmm29,xmm28
  65fde2:	vpandq xmm11,xmm28,xmm0
  65fde8:	vpmovzxbq xmm11,xmm11
  65fded:	vpsllq xmm11,xmm11,0x39
  65fdf3:	vgf2p8affineqb xmm24,xmm28,xmm6,0x0
  65fdfa:	vpandq xmm24,xmm24,xmm0
  65fe00:	vpmovzxbq xmm24,xmm24
  65fe06:	vpsllq xmm24,xmm24,0x21
  65fe0d:	vgf2p8affineqb xmm25,xmm28,xmm8,0x0
  65fe14:	vpandq xmm25,xmm25,xmm0
  65fe1a:	vpmovzxbq xmm25,xmm25
  65fe20:	vpsllq xmm25,xmm25,0x19
  65fe27:	vgf2p8affineqb xmm29,xmm28,xmm10,0x0
  65fe2e:	vpandq xmm29,xmm29,xmm0
  65fe34:	vpmovzxbq xmm29,xmm29
  65fe3a:	vmovdqa64 xmm19,xmm13
  65fe40:	vpsllq xmm13,xmm29,0x11
  65fe47:	vpternlogq xmm13,xmm25,xmm24,0xfe
  65fe4e:	vgf2p8affineqb xmm24,xmm28,xmm12,0x0
  65fe55:	vpandq xmm25,xmm24,xmm0
  65fe5b:	vpmovzxbq xmm25,xmm25
  65fe61:	vpsllq xmm25,xmm25,0x9
  65fe68:	vpandq xmm24,xmm24,xmm20
  65fe6e:	vmovdqa64 xmm17,xmm20
  65fe74:	vmovdqa64 xmm20,xmm18
  65fe7a:	vmovdqa64 xmm18,xmm14
  65fe80:	vpmovzxbq xmm14,xmm24
  65fe86:	vpternlogq xmm14,xmm11,xmm9,0xfe
  65fe8d:	vpunpcklbw xmm29,xmm3,xmm1
  65fe93:	vgf2p8affineqb xmm1,xmm29,xmm2,0x0
  65fe9a:	vpand  xmm1,xmm1,xmm0
  65fe9e:	vpmovzxbq xmm1,xmm1
  65fea3:	vpsllq xmm9,xmm1,0x32
  65fea8:	vpternlogq xmm9,xmm13,xmm25,0xfe
  65feaf:	vgf2p8affineqb xmm1,xmm29,xmm6,0x0
  65feb6:	vpand  xmm1,xmm1,xmm0
  65feba:	vpmovzxbq xmm1,xmm1
  65febf:	vpsllq xmm1,xmm1,0x22
  65fec4:	vgf2p8affineqb xmm3,xmm29,xmm8,0x0
  65fecb:	vpand  xmm3,xmm3,xmm0
  65fecf:	vpmovzxbq xmm3,xmm3
  65fed4:	vpsllq xmm3,xmm3,0x1a
  65fed9:	vgf2p8affineqb xmm13,xmm29,xmm10,0x0
  65fee0:	vpand  xmm11,xmm13,xmm0
  65fee4:	vpmovzxbq xmm11,xmm11
  65fee9:	vpsllq xmm11,xmm11,0x12
  65feef:	vpternlogq xmm11,xmm3,xmm1,0xfe
  65fef6:	vmovq  xmm24,rdx
  65fefc:	vpandq xmm1,xmm29,xmm0
  65ff02:	vpmovzxbq xmm1,xmm1
  65ff07:	vpsllq xmm3,xmm1,0x3a
  65ff0c:	vpandd xmm1,xmm13,DWORD PTR [rip+0xffffffffff9ac7d2]{1to4}        # c6e8 <anon.0fac96d756d24b1a23b798945b2a13ad.103.llvm.5929972809416623235+0x9c>
  65ff16:	vpmovzxbq xmm1,xmm1
  65ff1b:	vpternlogq xmm1,xmm14,xmm3,0xfe
  65ff22:	vmovq  xmm25,rax
  65ff28:	shrx   rax,QWORD PTR [rsp+0xd0],rcx
  65ff32:	cmove  rax,QWORD PTR [rsp+0x110]
  65ff3b:	shrx   rdx,QWORD PTR [rsp+0x150],rcx
  65ff45:	cmove  rdx,QWORD PTR [rsp+0x190]
  65ff4e:	vgf2p8affineqb xmm3,xmm28,xmm2,0x0
  65ff55:	vpand  xmm3,xmm3,xmm0
  65ff59:	vpmovzxbq xmm3,xmm3
  65ff5e:	vpsllq xmm3,xmm3,0x31
  65ff63:	vgf2p8affineqb xmm13,xmm28,xmm4,0x0
  65ff6a:	vpand  xmm13,xmm13,xmm0
  65ff6e:	vpmovzxbq xmm13,xmm13
  65ff73:	vpsllq xmm13,xmm13,0x29
  65ff79:	vpternlogq xmm1,xmm13,xmm3,0xfe
  65ff80:	vmovq  xmm3,rdx
  65ff85:	vgf2p8affineqb xmm5,xmm5,xmm12,0x0
  65ff8b:	vpand  xmm5,xmm5,xmm0
  65ff8f:	vpshufb xmm5,xmm5,XMMWORD PTR [rip+0xffffffffff9aca28]        # c9c0 <anon.0e5291c933a25680de2bf032d375ada2.27.llvm.16073103944965275176+0x10>
  65ff98:	vgf2p8affineqb xmm13,xmm29,xmm4,0x0
  65ff9f:	vpand  xmm13,xmm13,xmm0
  65ffa3:	vpmovzxbq xmm13,xmm13
  65ffa8:	vpsllq xmm13,xmm13,0x2a
  65ffae:	vgf2p8affineqb xmm14,xmm29,xmm12,0x0
  65ffb5:	vpand  xmm14,xmm14,xmm0
  65ffb9:	vpmovzxbq xmm14,xmm14
  65ffbe:	vpsllq xmm14,xmm14,0xa
  65ffc4:	vpternlogq xmm1,xmm9,xmm13,0xfe
  65ffcb:	vpternlogq xmm1,xmm7,xmm5,0xfe
  65ffd2:	vpunpcklbw xmm22,xmm23,xmm22
  65ffd8:	vpandq xmm5,xmm22,xmm0
  65ffde:	vpmovzxbq xmm5,xmm5
  65ffe3:	vpsllq xmm23,xmm5,0x3b
  65ffea:	vpternlogq xmm23,xmm11,xmm14,0xfe
  65fff1:	vgf2p8affineqb xmm5,xmm22,xmm2,0x0
  65fff8:	vpand  xmm5,xmm5,xmm0
  65fffc:	vpmovzxbq xmm5,xmm5
  660001:	vpsllq xmm5,xmm5,0x33
  660006:	vgf2p8affineqb xmm7,xmm22,xmm4,0x0
  66000d:	vpand  xmm7,xmm7,xmm0
  660011:	vpmovzxbq xmm7,xmm7
  660016:	vpsllq xmm7,xmm7,0x2b
  66001b:	vgf2p8affineqb xmm9,xmm22,xmm6,0x0
  660022:	vpand  xmm9,xmm9,xmm0
  660026:	vpmovzxbq xmm9,xmm9
  66002b:	vpsllq xmm9,xmm9,0x23
  660031:	vpternlogq xmm9,xmm7,xmm5,0xfe
  660038:	vgf2p8affineqb xmm29,xmm22,xmm8,0x0
  66003f:	vpandq xmm5,xmm29,xmm0
  660045:	vpmovzxbq xmm5,xmm5
  66004a:	vpsllq xmm5,xmm5,0x1b
  66004f:	vgf2p8affineqb xmm7,xmm22,xmm10,0x0
  660056:	vpand  xmm7,xmm7,xmm0
  66005a:	vpmovzxbq xmm7,xmm7
  66005f:	vpsllq xmm28,xmm7,0x13
  660066:	vpternlogq xmm28,xmm9,xmm5,0xfe
  66006d:	vpunpcklbw xmm26,xmm27,xmm26
  660073:	vpandq xmm5,xmm26,xmm0
  660079:	vpmovzxbq xmm5,xmm5
  66007e:	vpsllq xmm5,xmm5,0x3c
  660083:	vgf2p8affineqb xmm7,xmm26,xmm2,0x0
  66008a:	vpand  xmm7,xmm7,xmm0
  66008e:	vpmovzxbq xmm7,xmm7
  660093:	vpsllq xmm7,xmm7,0x34
  660098:	vgf2p8affineqb xmm9,xmm26,xmm4,0x0
  66009f:	vpand  xmm9,xmm9,xmm0
  6600a3:	vpmovzxbq xmm9,xmm9
  6600a8:	vpsllq xmm9,xmm9,0x2c
  6600ae:	vgf2p8affineqb xmm11,xmm26,xmm6,0x0
  6600b5:	vpand  xmm13,xmm11,xmm0
  6600b9:	vpmovzxbq xmm13,xmm13
  6600be:	vpsllq xmm13,xmm13,0x24
  6600c4:	vpandd xmm11,xmm11,DWORD PTR [rip+0xffffffffff9ac5d6]{1to4}        # c6a4 <anon.0fac96d756d24b1a23b798945b2a13ad.103.llvm.5929972809416623235+0x58>
  6600ce:	vpmovzxbq xmm11,xmm11
  6600d3:	vporq  xmm27,xmm5,xmm11
  6600d9:	vgf2p8affineqb xmm5,xmm26,xmm8,0x0
  6600e0:	vpand  xmm5,xmm5,xmm0
  6600e4:	vpmovzxbq xmm5,xmm5
  6600e9:	vpsllq xmm5,xmm5,0x1c
  6600ee:	vpternlogq xmm27,xmm9,xmm7,0xfe
  6600f5:	vgf2p8affineqb xmm7,xmm26,xmm12,0x0
  6600fc:	vpand  xmm7,xmm7,xmm0
  660100:	vpmovzxbq xmm7,xmm7
  660105:	vpsllq xmm9,xmm7,0xc
  66010a:	vpternlogq xmm27,xmm5,xmm13,0xfe
  660111:	vpunpcklbw xmm5,xmm31,xmm21
  660117:	vpand  xmm7,xmm5,xmm0
  66011b:	vpmovzxbq xmm7,xmm7
  660120:	vpsllq xmm11,xmm7,0x3d
  660125:	vgf2p8affineqb xmm7,xmm5,xmm8,0x0
  66012b:	vpand  xmm7,xmm7,xmm0
  66012f:	vpmovzxbq xmm7,xmm7
  660134:	vpsllq xmm7,xmm7,0x1d
  660139:	vgf2p8affineqb xmm13,xmm5,xmm10,0x0
  66013f:	vpand  xmm13,xmm13,xmm0
  660143:	vpmovzxbq xmm13,xmm13
  660148:	vpsllq xmm13,xmm13,0x15
  66014e:	vgf2p8affineqb xmm14,xmm5,xmm12,0x0
  660154:	vpand  xmm14,xmm14,xmm0
  660158:	vpmovzxbq xmm14,xmm14
  66015d:	vpsllq xmm14,xmm14,0xd
  660163:	vpternlogq xmm14,xmm13,xmm7,0xfe
  66016a:	vgf2p8affineqb xmm13,xmm5,xmm4,0x0
  660170:	vpandq xmm7,xmm13,xmm30
  660176:	vpmovzxbq xmm7,xmm7
  66017b:	vpternlogq xmm7,xmm11,xmm9,0xfe
  660182:	vgf2p8affineqb xmm9,xmm5,xmm2,0x0
  660188:	vpand  xmm9,xmm9,xmm0
  66018c:	vpmovzxbq xmm9,xmm9
  660191:	vpsllq xmm9,xmm9,0x35
  660197:	vpand  xmm11,xmm13,xmm0
  66019b:	vpmovzxbq xmm11,xmm11
  6601a0:	vpsllq xmm11,xmm11,0x2d
  6601a6:	vpternlogq xmm7,xmm11,xmm9,0xfe
  6601ad:	vpunpcklbw xmm13,xmm25,xmm24
  6601b3:	vgf2p8affineqb xmm24,xmm13,xmm2,0x0
  6601ba:	vpandq xmm9,xmm24,xmm0
  6601c0:	vpmovzxbq xmm9,xmm9
  6601c5:	vpsllq xmm9,xmm9,0x36
  6601cb:	vgf2p8affineqb xmm11,xmm13,xmm4,0x0
  6601d1:	vpand  xmm11,xmm11,xmm0
  6601d5:	vpmovzxbq xmm11,xmm11
  6601da:	vpsllq xmm11,xmm11,0x2e
  6601e0:	vpor   xmm9,xmm11,xmm9
  6601e5:	vgf2p8affineqb xmm11,xmm13,xmm6,0x0
  6601eb:	vpand  xmm11,xmm11,xmm0
  6601ef:	vpmovzxbq xmm11,xmm11
  6601f4:	vpsllq xmm11,xmm11,0x26
  6601fa:	vgf2p8affineqb xmm25,xmm13,xmm8,0x0
  660201:	vpandq xmm25,xmm25,xmm0
  660207:	vpmovzxbq xmm25,xmm25
  66020d:	vpsllq xmm25,xmm25,0x1e
  660214:	vpternlogq xmm25,xmm9,xmm11,0xfe
  66021b:	vgf2p8affineqb xmm9,xmm13,xmm10,0x0
  660221:	vpand  xmm9,xmm9,xmm0
  660225:	vpmovzxbq xmm9,xmm9
  66022a:	vpsllq xmm11,xmm9,0x16
  660230:	vgf2p8affineqb xmm9,xmm13,xmm12,0x0
  660236:	vpand  xmm9,xmm9,xmm0
  66023a:	vpmovzxbq xmm9,xmm9
  66023f:	vpsllq xmm9,xmm9,0xe
  660245:	vpternlogq xmm9,xmm25,xmm11,0xfe
  66024c:	vmovq  xmm11,rax
  660251:	mov    rax,QWORD PTR [rsp+0x50]
  660256:	vpand  xmm13,xmm13,xmm0
  66025a:	vpmovzxbq xmm13,xmm13
  66025f:	vpsllq xmm13,xmm13,0x3e
  660265:	vpandq xmm24,xmm24,xmm16
  66026b:	vpmovzxbq xmm24,xmm24
  660271:	vpternlogq xmm24,xmm14,xmm13,0xfe
  660278:	vpunpcklbw xmm13,xmm11,xmm3
  66027c:	vgf2p8affineqb xmm14,xmm13,xmm2,0x0
  660282:	vpand  xmm14,xmm14,xmm0
  660286:	vpmovzxbq xmm14,xmm14
  66028b:	vpsllq xmm14,xmm14,0x37
  660291:	vgf2p8affineqb xmm25,xmm13,xmm4,0x0
  660298:	vpandq xmm25,xmm25,xmm0
  66029e:	vpmovzxbq xmm25,xmm25
  6602a4:	vpsllq xmm25,xmm25,0x2f
  6602ab:	vpternlogq xmm25,xmm1,xmm14,0xfe
  6602b2:	vgf2p8affineqb xmm1,xmm13,xmm6,0x0
  6602b8:	vpand  xmm1,xmm1,xmm0
  6602bc:	vpmovzxbq xmm1,xmm1
  6602c1:	vpsllq xmm1,xmm1,0x27
  6602c6:	vgf2p8affineqb xmm14,xmm13,xmm8,0x0
  6602cc:	vpand  xmm14,xmm14,xmm0
  6602d0:	vpmovzxbq xmm14,xmm14
  6602d5:	vpsllq xmm14,xmm14,0x1f
  6602db:	vpternlogq xmm14,xmm25,xmm1,0xfe
  6602e2:	vgf2p8affineqb xmm1,xmm13,xmm10,0x0
  6602e8:	vpand  xmm1,xmm1,xmm0
  6602ec:	vpmovzxbq xmm1,xmm1
  6602f1:	vpsllq xmm1,xmm1,0x17
  6602f6:	vgf2p8affineqb xmm25,xmm13,xmm12,0x0
  6602fd:	vpandq xmm25,xmm25,xmm0
  660303:	vpmovzxbq xmm25,xmm25
  660309:	vpsllq xmm25,xmm25,0xf
  660310:	vpternlogq xmm25,xmm14,xmm1,0xfe
  660317:	vmovdqa64 xmm14,xmm18
  66031d:	vmovdqa64 xmm18,xmm20
  660323:	vmovdqa64 xmm20,xmm17
  660329:	vpandq xmm1,xmm29,xmm15
  66032f:	vpmovzxbq xmm1,xmm1
  660334:	vpternlogq xmm25,xmm23,xmm1,0xfe
  66033b:	vgf2p8affineqb xmm1,xmm22,xmm12,0x0
  660342:	vpbroadcastd xmm17,DWORD PTR [rip+0xffffffffff9ac398]        # c6e4 <anon.0fac96d756d24b1a23b798945b2a13ad.103.llvm.5929972809416623235+0x98>
  66034c:	vpand  xmm1,xmm1,xmm0
  660350:	vpmovzxbq xmm1,xmm1
  660355:	vpsllq xmm1,xmm1,0xb
  66035a:	vpternlogq xmm25,xmm28,xmm1,0xfe
  660361:	vgf2p8affineqb xmm1,xmm5,xmm6,0x0
  660367:	vpand  xmm1,xmm1,xmm0
  66036b:	vpmovzxbq xmm1,xmm1
  660370:	vpsllq xmm1,xmm1,0x25
  660375:	vpternlogq xmm25,xmm7,xmm1,0xfe
  66037c:	vgf2p8affineqb xmm1,xmm26,xmm10,0x0
  660383:	vpand  xmm1,xmm1,xmm0
  660387:	vpmovzxbq xmm1,xmm1
  66038c:	vpsllq xmm1,xmm1,0x14
  660391:	vpternlogq xmm25,xmm27,xmm1,0xfe
  660398:	vpandq xmm1,xmm13,xmm17
  66039e:	vmovdqa64 xmm13,xmm19
  6603a4:	vmovdqa64 xmm17,XMMWORD PTR [rip+0xffffffffff9ac6f2]        # caa0 <anon.0e5291c933a25680de2bf032d375ada2.27.llvm.16073103944965275176+0xf0>
  6603ae:	vpbroadcastq xmm19,QWORD PTR [rip+0xffffffffff9ac8f0]        # cca8 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0x28>
  6603b8:	vpmovzxbq xmm1,xmm1
  6603bd:	vpternlogq xmm1,xmm24,xmm25,0xfe
  6603c4:	vpunpcklqdq xmm3,xmm11,xmm3
  6603c8:	vmovdqa xmm11,XMMWORD PTR [rip+0xffffffffff9ac6a0]        # ca70 <anon.0e5291c933a25680de2bf032d375ada2.27.llvm.16073103944965275176+0xc0>
  6603d0:	vpsllq xmm3,xmm3,0x3f
  6603d5:	vpternlogq xmm1,xmm9,xmm3,0xfe
  6603dc:	vmovdqa xmm9,XMMWORD PTR [rip+0xffffffffff9ac4cc]        # c8b0 <anon.60695053c5df2db48177e894d4d8ab59.354.llvm.16546105846116086659+0xf0>
  6603e4:	vmovdqu XMMWORD PTR [rsp+rax*8+0x590],xmm1
  6603ed:	add    rax,0x2
  6603f1:	add    rsi,0x2000
  6603f8:	cmp    rax,0x10
  6603fc:	jne    65fab0 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x26e0>
  660402:	vmovdqu ymm1,YMMWORD PTR [rsp+0x590]
  66040b:	vmovdqu ymm3,YMMWORD PTR [rsp+0x5b0]
  660414:	vmovdqu ymm5,YMMWORD PTR [rsp+0x5d0]
  66041d:	vmovdqu ymm7,YMMWORD PTR [rsp+0x5f0]
  660426:	mov    rcx,QWORD PTR [rsp+0x20]
  66042b:	mov    rax,rcx
  66042e:	shl    rax,0x7
  660432:	vmovdqu YMMWORD PTR [rsp+rax*1+0x2590],ymm1
  66043b:	vmovdqu YMMWORD PTR [rsp+rax*1+0x25b0],ymm3
  660444:	vmovdqu YMMWORD PTR [rsp+rax*1+0x25d0],ymm5
  66044d:	vmovdqu YMMWORD PTR [rsp+rax*1+0x25f0],ymm7
  660456:	inc    rcx
  660459:	cmp    rcx,0x10
  66045d:	jne    65fa90 <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x26c0>
  660463:	lea    rdi,[rsp+0x1590]
  66046b:	vzeroupper 
  66046e:	call   670780 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5field11byte_tables>
  660473:	vmovups ymm0,YMMWORD PTR [rsp+0x570]
  66047c:	vmovups YMMWORD PTR [rsp+0x670],ymm0
  660485:	vmovups ymm0,YMMWORD PTR [rsp+0x550]
  66048e:	vmovups YMMWORD PTR [rsp+0x650],ymm0
  660497:	vmovups ymm0,YMMWORD PTR [rsp+0x530]
  6604a0:	vmovups YMMWORD PTR [rsp+0x630],ymm0
  6604a9:	vmovups ymm0,YMMWORD PTR [rsp+0x510]
  6604b2:	vmovups YMMWORD PTR [rsp+0x610],ymm0
  6604bb:	vmovups ymm0,YMMWORD PTR [rsp+0x490]
  6604c4:	vmovups ymm1,YMMWORD PTR [rsp+0x4b0]
  6604cd:	vmovups ymm2,YMMWORD PTR [rsp+0x4d0]
  6604d6:	vmovups ymm3,YMMWORD PTR [rsp+0x4f0]
  6604df:	vmovups YMMWORD PTR [rsp+0x5f0],ymm3
  6604e8:	vmovups YMMWORD PTR [rsp+0x5d0],ymm2
  6604f1:	vmovups YMMWORD PTR [rsp+0x5b0],ymm1
  6604fa:	vmovups YMMWORD PTR [rsp+0x590],ymm0
  660503:	mov    rdi,QWORD PTR [rsp+0x10]
  660508:	mov    rcx,QWORD PTR [rsp+0x8]
  66050d:	mov    QWORD PTR [rdi],rcx
  660510:	mov    QWORD PTR [rdi+0x8],0x10000
  660518:	mov    rcx,QWORD PTR [rsp]
  66051c:	mov    QWORD PTR [rdi+0x10],rcx
  660520:	mov    QWORD PTR [rdi+0x18],0x10
  660528:	mov    QWORD PTR [rdi+0x20],rax
  66052c:	mov    QWORD PTR [rdi+0x28],0x10
  660534:	vmovups ymm0,YMMWORD PTR [rsp+0x590]
  66053d:	vmovups ymm1,YMMWORD PTR [rsp+0x5b0]
  660546:	vmovups ymm2,YMMWORD PTR [rsp+0x5d0]
  66054f:	vmovups ymm3,YMMWORD PTR [rsp+0x5f0]
  660558:	vmovups YMMWORD PTR [rdi+0x30],ymm0
  66055d:	vmovups YMMWORD PTR [rdi+0x50],ymm1
  660562:	vmovups YMMWORD PTR [rdi+0x70],ymm2
  660567:	vmovups YMMWORD PTR [rdi+0x90],ymm3
  66056f:	vmovups ymm0,YMMWORD PTR [rsp+0x610]
  660578:	vmovups YMMWORD PTR [rdi+0xb0],ymm0
  660580:	vmovups ymm0,YMMWORD PTR [rsp+0x630]
  660589:	vmovups YMMWORD PTR [rdi+0xd0],ymm0
  660591:	vmovups ymm0,YMMWORD PTR [rsp+0x650]
  66059a:	vmovups YMMWORD PTR [rdi+0xf0],ymm0
  6605a2:	vmovups ymm0,YMMWORD PTR [rsp+0x670]
  6605ab:	vmovups YMMWORD PTR [rdi+0x110],ymm0
  6605b3:	add    rdi,0x130
  6605ba:	lea    rsi,[rsp+0x2590]
  6605c2:	mov    edx,0x800
  6605c7:	vzeroupper 
  6605ca:	call   QWORD PTR [rip+0x97c60]        # 6f8230 <memcpy@GLIBC_2.14>
  6605d0:	add    rsp,0x2e38
  6605d7:	pop    rbx
  6605d8:	pop    r12
  6605da:	pop    r13
  6605dc:	pop    r14
  6605de:	pop    r15
  6605e0:	pop    rbp
  6605e1:	ret    
  6605e2:	lea    rdi,[rip+0xffffffffff9b21af]        # 12798 <anon.0e5291c933a25680de2bf032d375ada2.76.llvm.16073103944965275176+0x95>
  6605e9:	lea    rdx,[rip+0x94e08]        # 6f53f8 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x678>
  6605f0:	mov    esi,0x3a
  6605f5:	call   QWORD PTR [rip+0x97eb5]        # 6f84b0 <_DYNAMIC+0x498>
  6605fb:	ud2    
  6605fd:	lea    rdi,[rip+0x947f4]        # 6f4df8 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x78>
  660604:	call   QWORD PTR [rip+0x97dae]        # 6f83b8 <_DYNAMIC+0x3a0>
  66060a:	mov    edi,0x1
  66060f:	mov    esi,0x10000
  660614:	call   QWORD PTR [rip+0x97c3e]        # 6f8258 <_DYNAMIC+0x240>
  66061a:	mov    rbx,rax
  66061d:	mov    rdi,QWORD PTR [rsp]
  660621:	call   QWORD PTR [rip+0x97bf9]        # 6f8220 <free@GLIBC_2.2.5>
  660627:	mov    rdi,QWORD PTR [rsp+0x8]
  66062c:	call   QWORD PTR [rip+0x97bee]        # 6f8220 <free@GLIBC_2.2.5>
  660632:	mov    rdi,rbx
  660635:	call   6f1930 <_Unwind_Resume@plt>
  66063a:	jmp    66063c <_RNCINvMs0_NtNtCs8srXsAgjQHr_3std4sync4onceNtB8_4Once15call_once_forceNCINvMNtBa_9once_lockINtB1b_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB1a_11get_or_initNvMB1I_B1G_3newE0zE0E0B1M_.llvm.16073103944965275176+0x326c>
  66063c:	mov    rbx,rax
  66063f:	mov    rdi,QWORD PTR [rsp+0x8]
  660644:	call   QWORD PTR [rip+0x97bd6]        # 6f8220 <free@GLIBC_2.2.5>
  66064a:	mov    rdi,rbx
  66064d:	call   6f1930 <_Unwind_Resume@plt>
  660652:	int3   
  660653:	int3   
  660654:	int3   
  660655:	int3   
  660656:	int3   
  660657:	int3   
  660658:	int3   
  660659:	int3   
  66065a:	int3   
  66065b:	int3   
  66065c:	int3   
  66065d:	int3   
  66065e:	int3   
  66065f:	int3   

