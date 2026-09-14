000000000066f120 <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into>:
  66f120:	push   rbp
  66f121:	push   r15
  66f123:	push   r14
  66f125:	push   r13
  66f127:	push   r12
  66f129:	push   rbx
  66f12a:	sub    rsp,0x168
  66f131:	mov    QWORD PTR [rsp+0x18],rdx
  66f136:	mov    QWORD PTR [rsp+0x20],r8
  66f13b:	cmp    rdx,r8
  66f13e:	jne    66f4f9 <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0x3d9>
  66f144:	mov    rbp,rdx
  66f147:	mov    QWORD PTR [rsp+0x10],rdi
  66f14c:	mov    r14,QWORD PTR [rsp+0x1b0]
  66f154:	mov    QWORD PTR [rsp+0x48],rdx
  66f159:	mov    rax,r14
  66f15c:	shl    rax,0x4
  66f160:	mov    QWORD PTR [rsp+0x28],rax
  66f165:	cmp    rdx,rax
  66f168:	jne    66f514 <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0x3f4>
  66f16e:	mov    rbx,rcx
  66f171:	mov    r15,rsi
  66f174:	mov    r12,QWORD PTR [rsp+0x1b8]
  66f17c:	mov    rdx,QWORD PTR [rsp+0x1a0]
  66f184:	lea    rdi,[rsp+0x30]
  66f189:	mov    rsi,r9
  66f18c:	call   666c70 <_RNvMNtCs5Palesvl9wF_6mikros6lookupNtB2_12BindingTable3new>
  66f191:	mov    QWORD PTR [r12+0x10],0x0
  66f19a:	mov    rdx,QWORD PTR [r12]
  66f19e:	cmp    r14,rdx
  66f1a1:	ja     66f52f <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0x40f>
  66f1a7:	xor    r13d,r13d
  66f1aa:	sub    rdx,r13
  66f1ad:	cmp    r14,rdx
  66f1b0:	ja     66f58b <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0x46b>
  66f1b6:	mov    rcx,QWORD PTR [rsp+0x1c0]
  66f1be:	shl    r13,0x4
  66f1c2:	mov    rax,QWORD PTR [rsp+0x1b8]
  66f1ca:	add    r13,QWORD PTR [rax+0x8]
  66f1ce:	mov    QWORD PTR [rcx+0x10],0x0
  66f1d6:	mov    rdx,QWORD PTR [rcx]
  66f1d9:	cmp    r14,rdx
  66f1dc:	ja     66f55e <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0x43e>
  66f1e2:	xor    r12d,r12d
  66f1e5:	sub    rdx,r12
  66f1e8:	cmp    r14,rdx
  66f1eb:	ja     66f58b <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0x46b>
  66f1f1:	mov    rax,QWORD PTR [rsp+0x1a8]
  66f1f9:	shl    r12,0x4
  66f1fd:	mov    rcx,QWORD PTR [rsp+0x1c0]
  66f205:	add    r12,QWORD PTR [rcx+0x8]
  66f209:	lea    rcx,[rsp+0x30]
  66f20e:	mov    QWORD PTR [rsp+0x80],rcx
  66f216:	mov    QWORD PTR [rsp+0x88],r15
  66f21e:	mov    QWORD PTR [rsp+0x90],rbp
  66f226:	mov    QWORD PTR [rsp+0x98],rbx
  66f22e:	mov    QWORD PTR [rsp+0xa0],rbp
  66f236:	mov    QWORD PTR [rsp+0xa8],rax
  66f23e:	mov    QWORD PTR [rsp+0xb0],r14
  66f246:	cmp    r14,0x1000
  66f24d:	jb     66f27a <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0x15a>
  66f24f:	mov    r15,0xffffffffffffffa8
  66f256:	mov    rax,QWORD PTR fs:[r15]
  66f25a:	test   rax,rax
  66f25d:	je     66f267 <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0x147>
  66f25f:	add    rax,0x110
  66f265:	jmp    66f26d <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0x14d>
  66f267:	call   QWORD PTR [rip+0x89373]        # 6f85e0 <_DYNAMIC+0x5c8>
  66f26d:	mov    rax,QWORD PTR [rax]
  66f270:	cmp    QWORD PTR [rax+0x208],0x1
  66f278:	jne    66f2a3 <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0x183>
  66f27a:	mov    QWORD PTR [rsp],r14
  66f27e:	lea    rdi,[rsp+0x128]
  66f286:	lea    rsi,[rsp+0x80]
  66f28e:	xor    edx,edx
  66f290:	mov    rcx,r13
  66f293:	mov    r8,r14
  66f296:	mov    r9,r12
  66f299:	call   6650d0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_>
  66f29e:	jmp    66f3b8 <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0x298>
  66f2a3:	mov    QWORD PTR [rsp+0xb8],r13
  66f2ab:	mov    QWORD PTR [rsp+0xc0],r14
  66f2b3:	mov    QWORD PTR [rsp+0xc8],0x400
  66f2bf:	mov    QWORD PTR [rsp+0xd0],r12
  66f2c7:	mov    QWORD PTR [rsp+0xd8],r14
  66f2cf:	mov    QWORD PTR [rsp+0xe0],0x400
  66f2db:	lea    rax,[rsp+0xe8]
  66f2e3:	lea    rcx,[rsp+0x80]
  66f2eb:	mov    QWORD PTR [rsp+0xe8],rcx
  66f2f3:	lea    rbx,[r14+0x3ff]
  66f2fa:	shr    rbx,0xa
  66f2fe:	mov    QWORD PTR [rsp+0xf0],r13
  66f306:	mov    QWORD PTR [rsp+0xf8],r14
  66f30e:	mov    QWORD PTR [rsp+0x100],0x400
  66f31a:	lea    rcx,[rsp+0xf]
  66f31f:	mov    QWORD PTR [rsp+0x108],rcx
  66f327:	mov    QWORD PTR [rsp+0x110],rcx
  66f32f:	mov    QWORD PTR [rsp+0x118],rax
  66f337:	mov    QWORD PTR [rsp+0x120],rbx
  66f33f:	mov    QWORD PTR [rsp+0x48],r13
  66f344:	mov    QWORD PTR [rsp+0x50],r14
  66f349:	mov    QWORD PTR [rsp+0x58],0x400
  66f352:	mov    QWORD PTR [rsp+0x60],r12
  66f357:	mov    QWORD PTR [rsp+0x68],r14
  66f35c:	mov    QWORD PTR [rsp+0x70],0x400
  66f365:	mov    QWORD PTR [rsp+0x78],0x0
  66f36e:	mov    rax,QWORD PTR fs:[r15]
  66f372:	test   rax,rax
  66f375:	je     66f37f <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0x25f>
  66f377:	add    rax,0x110
  66f37d:	jmp    66f385 <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0x265>
  66f37f:	call   QWORD PTR [rip+0x8925b]        # 6f85e0 <_DYNAMIC+0x5c8>
  66f385:	lea    rdx,[rsp+0x108]
  66f38d:	mov    rax,QWORD PTR [rax]
  66f390:	mov    rcx,QWORD PTR [rax+0x208]
  66f397:	mov    QWORD PTR [rsp],rdx
  66f39b:	lea    rdi,[rsp+0x128]
  66f3a3:	lea    r9,[rsp+0x48]
  66f3a8:	mov    r8d,0x1
  66f3ae:	mov    rsi,rbx
  66f3b1:	xor    edx,edx
  66f3b3:	call   65ca30 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_>
  66f3b8:	mov    rax,QWORD PTR [rsp+0x1b8]
  66f3c0:	mov    QWORD PTR [rax+0x10],r14
  66f3c4:	vmovdqa ymm0,YMMWORD PTR [rip+0xffffffffff99df94]        # d360 <anon.36d21461a6996b7f2bd9a76e3225918f.28.llvm.7983859039285366149+0x1c0>
  66f3cc:	vpermq zmm1,zmm0,ZMMWORD PTR [rsp+0x128]
  66f3d7:	vmovdqu ymm2,YMMWORD PTR [rsp+0x128]
  66f3e0:	vmovdqu ymm3,YMMWORD PTR [rsp+0x148]
  66f3e9:	mov    rax,QWORD PTR [rsp+0x1c0]
  66f3f1:	mov    QWORD PTR [rax+0x10],r14
  66f3f5:	vpunpcklqdq ymm4,ymm2,ymm3
  66f3f9:	vextracti128 xmm4,ymm4,0x1
  66f3ff:	vpaddq xmm5,xmm4,xmm4
  66f403:	vpunpckhqdq ymm6,ymm2,ymm3
  66f407:	vextracti128 xmm6,ymm6,0x1
  66f40d:	vpshldq xmm7,xmm6,xmm4,0x1
  66f414:	vpsllq xmm8,xmm4,0x2
  66f419:	vpshldq xmm9,xmm6,xmm4,0x2
  66f420:	vpsllq xmm10,xmm4,0x7
  66f425:	vpsrlvq xmm11,xmm6,XMMWORD PTR [rip+0xffffffffff99d532]        # c960 <anon.60695053c5df2db48177e894d4d8ab59.354.llvm.16546105846116086659+0x1a0>
  66f42e:	vpshldq xmm4,xmm6,xmm4,0x7
  66f435:	vpsrlvq xmm12,xmm6,XMMWORD PTR [rip+0xffffffffff99d4f2]        # c930 <anon.60695053c5df2db48177e894d4d8ab59.354.llvm.16546105846116086659+0x170>
  66f43e:	vpsrlq xmm6,xmm6,0x39
  66f443:	vpternlogq xmm6,xmm11,xmm12,0x96
  66f44a:	vinserti128 ymm11,ymm6,xmm6,0x1
  66f450:	vmovdqa ymm12,YMMWORD PTR [rip+0xffffffffff99e268]        # d6c0 <anon.36d21461a6996b7f2bd9a76e3225918f.28.llvm.7983859039285366149+0x520>
  66f458:	vmovdqa ymm13,ymm11
  66f45d:	vpermt2q ymm13,ymm12,ymm9
  66f463:	vpsllvq ymm9,ymm13,YMMWORD PTR [rip+0xffffffffff99dc34]        # d0a0 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0x420>
  66f46c:	vpsllq xmm13,xmm6,0x2
  66f471:	vpermt2q ymm11,ymm12,ymm7
  66f477:	vpsllvq ymm7,ymm11,YMMWORD PTR [rip+0xffffffffff99dcc0]        # d140 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0x4c0>
  66f480:	vpternlogq xmm1,xmm5,xmm8,0x96
  66f487:	vpternlogq xmm1,xmm13,xmm10,0x96
  66f48e:	vperm2i128 ymm5,ymm2,ymm3,0x20
  66f494:	vmovdqa ymm8,YMMWORD PTR [rip+0xffffffffff99e384]        # d820 <anon.36d21461a6996b7f2bd9a76e3225918f.28.llvm.7983859039285366149+0x680>
  66f49c:	vpermt2q ymm1,ymm8,ymm5
  66f4a2:	vpternlogq ymm1,ymm7,ymm9,0x96
  66f4a9:	vperm2i128 ymm2,ymm2,ymm3,0x31
  66f4af:	vpermi2q ymm0,ymm2,ymm4
  66f4b5:	vpermi2q ymm8,ymm6,ymm2
  66f4bb:	vpternlogq ymm8,ymm1,ymm0,0x96
  66f4c2:	mov    rbx,QWORD PTR [rsp+0x10]
  66f4c7:	vmovdqu YMMWORD PTR [rbx],ymm8
  66f4cb:	cmp    QWORD PTR [rsp+0x30],0x0
  66f4d1:	je     66f4e1 <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0x3c1>
  66f4d3:	mov    rdi,QWORD PTR [rsp+0x38]
  66f4d8:	vzeroupper 
  66f4db:	call   QWORD PTR [rip+0x88d3f]        # 6f8220 <free@GLIBC_2.2.5>
  66f4e1:	mov    rax,rbx
  66f4e4:	add    rsp,0x168
  66f4eb:	pop    rbx
  66f4ec:	pop    r12
  66f4ee:	pop    r13
  66f4f0:	pop    r14
  66f4f2:	pop    r15
  66f4f4:	pop    rbp
  66f4f5:	vzeroupper 
  66f4f8:	ret    
  66f4f9:	lea    r9,[rip+0x85dd8]        # 6f52d8 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x558>
  66f500:	lea    rsi,[rsp+0x18]
  66f505:	lea    rdx,[rsp+0x20]
  66f50a:	xor    edi,edi
  66f50c:	xor    ecx,ecx
  66f50e:	call   QWORD PTR [rip+0x88ecc]        # 6f83e0 <_DYNAMIC+0x3c8>
  66f514:	lea    r9,[rip+0x85dd5]        # 6f52f0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x570>
  66f51b:	lea    rsi,[rsp+0x48]
  66f520:	lea    rdx,[rsp+0x28]
  66f525:	xor    edi,edi
  66f527:	xor    ecx,ecx
  66f529:	call   QWORD PTR [rip+0x88eb1]        # 6f83e0 <_DYNAMIC+0x3c8>
  66f52f:	mov    rdi,QWORD PTR [rsp+0x1b8]
  66f537:	xor    esi,esi
  66f539:	mov    rdx,r14
  66f53c:	call   6574d0 <_RINvNvMs2_NtCs3KfQW3d0ssT_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs5Palesvl9wF_6mikros>
  66f541:	mov    rax,QWORD PTR [rsp+0x1b8]
  66f549:	mov    rdx,QWORD PTR [rax]
  66f54c:	mov    r13,QWORD PTR [rax+0x10]
  66f550:	sub    rdx,r13
  66f553:	cmp    r14,rdx
  66f556:	jbe    66f1b6 <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0x96>
  66f55c:	jmp    66f58b <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0x46b>
  66f55e:	mov    rdi,QWORD PTR [rsp+0x1c0]
  66f566:	xor    esi,esi
  66f568:	mov    rdx,r14
  66f56b:	call   6574d0 <_RINvNvMs2_NtCs3KfQW3d0ssT_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs5Palesvl9wF_6mikros>
  66f570:	mov    rax,QWORD PTR [rsp+0x1c0]
  66f578:	mov    rdx,QWORD PTR [rax]
  66f57b:	mov    r12,QWORD PTR [rax+0x10]
  66f57f:	sub    rdx,r12
  66f582:	cmp    r14,rdx
  66f585:	jbe    66f1f1 <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0xd1>
  66f58b:	lea    rcx,[rip+0x85bae]        # 6f5140 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x3c0>
  66f592:	xor    edi,edi
  66f594:	mov    rsi,r14
  66f597:	call   QWORD PTR [rip+0x88dbb]        # 6f8358 <_DYNAMIC+0x340>
  66f59d:	ud2    
  66f59f:	mov    rbx,rax
  66f5a2:	cmp    QWORD PTR [rsp+0x30],0x0
  66f5a8:	je     66f5b5 <_RNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into+0x495>
  66f5aa:	mov    rdi,QWORD PTR [rsp+0x38]
  66f5af:	call   QWORD PTR [rip+0x88c6b]        # 6f8220 <free@GLIBC_2.2.5>
  66f5b5:	mov    rdi,rbx
  66f5b8:	call   6f1930 <_Unwind_Resume@plt>
  66f5bd:	int3   
  66f5be:	int3   
  66f5bf:	int3   

