000000000066ec80 <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into>:
  66ec80:	push   rbp
  66ec81:	push   r15
  66ec83:	push   r14
  66ec85:	push   r13
  66ec87:	push   r12
  66ec89:	push   rbx
  66ec8a:	sub    rsp,0x168
  66ec91:	mov    QWORD PTR [rsp+0x18],rdx
  66ec96:	mov    QWORD PTR [rsp+0x20],r8
  66ec9b:	cmp    rdx,r8
  66ec9e:	jne    66f059 <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0x3d9>
  66eca4:	mov    rbp,rdx
  66eca7:	mov    QWORD PTR [rsp+0x10],rdi
  66ecac:	mov    r14,QWORD PTR [rsp+0x1b0]
  66ecb4:	mov    QWORD PTR [rsp+0x48],rdx
  66ecb9:	mov    rax,r14
  66ecbc:	shl    rax,0x4
  66ecc0:	mov    QWORD PTR [rsp+0x28],rax
  66ecc5:	cmp    rdx,rax
  66ecc8:	jne    66f074 <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0x3f4>
  66ecce:	mov    rbx,rcx
  66ecd1:	mov    r15,rsi
  66ecd4:	mov    r12,QWORD PTR [rsp+0x1b8]
  66ecdc:	mov    rdx,QWORD PTR [rsp+0x1a0]
  66ece4:	lea    rdi,[rsp+0x30]
  66ece9:	mov    rsi,r9
  66ecec:	call   666c70 <_RNvMNtCs5Palesvl9wF_6mikros6lookupNtB2_12BindingTable3new>
  66ecf1:	mov    QWORD PTR [r12+0x10],0x0
  66ecfa:	mov    rdx,QWORD PTR [r12]
  66ecfe:	cmp    r14,rdx
  66ed01:	ja     66f08f <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0x40f>
  66ed07:	xor    r13d,r13d
  66ed0a:	sub    rdx,r13
  66ed0d:	cmp    r14,rdx
  66ed10:	ja     66f0eb <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0x46b>
  66ed16:	mov    rcx,QWORD PTR [rsp+0x1c0]
  66ed1e:	shl    r13,0x4
  66ed22:	mov    rax,QWORD PTR [rsp+0x1b8]
  66ed2a:	add    r13,QWORD PTR [rax+0x8]
  66ed2e:	mov    QWORD PTR [rcx+0x10],0x0
  66ed36:	mov    rdx,QWORD PTR [rcx]
  66ed39:	cmp    r14,rdx
  66ed3c:	ja     66f0be <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0x43e>
  66ed42:	xor    r12d,r12d
  66ed45:	sub    rdx,r12
  66ed48:	cmp    r14,rdx
  66ed4b:	ja     66f0eb <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0x46b>
  66ed51:	mov    rax,QWORD PTR [rsp+0x1a8]
  66ed59:	shl    r12,0x4
  66ed5d:	mov    rcx,QWORD PTR [rsp+0x1c0]
  66ed65:	add    r12,QWORD PTR [rcx+0x8]
  66ed69:	lea    rcx,[rsp+0x30]
  66ed6e:	mov    QWORD PTR [rsp+0x80],rcx
  66ed76:	mov    QWORD PTR [rsp+0x88],r15
  66ed7e:	mov    QWORD PTR [rsp+0x90],rbp
  66ed86:	mov    QWORD PTR [rsp+0x98],rbx
  66ed8e:	mov    QWORD PTR [rsp+0xa0],rbp
  66ed96:	mov    QWORD PTR [rsp+0xa8],rax
  66ed9e:	mov    QWORD PTR [rsp+0xb0],r14
  66eda6:	cmp    r14,0x1000
  66edad:	jb     66edda <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0x15a>
  66edaf:	mov    r15,0xffffffffffffffa8
  66edb6:	mov    rax,QWORD PTR fs:[r15]
  66edba:	test   rax,rax
  66edbd:	je     66edc7 <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0x147>
  66edbf:	add    rax,0x110
  66edc5:	jmp    66edcd <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0x14d>
  66edc7:	call   QWORD PTR [rip+0x89813]        # 6f85e0 <_DYNAMIC+0x5c8>
  66edcd:	mov    rax,QWORD PTR [rax]
  66edd0:	cmp    QWORD PTR [rax+0x208],0x1
  66edd8:	jne    66ee03 <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0x183>
  66edda:	mov    QWORD PTR [rsp],r14
  66edde:	lea    rdi,[rsp+0x128]
  66ede6:	lea    rsi,[rsp+0x80]
  66edee:	xor    edx,edx
  66edf0:	mov    rcx,r13
  66edf3:	mov    r8,r14
  66edf6:	mov    r9,r12
  66edf9:	call   664440 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_>
  66edfe:	jmp    66ef18 <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0x298>
  66ee03:	mov    QWORD PTR [rsp+0xb8],r13
  66ee0b:	mov    QWORD PTR [rsp+0xc0],r14
  66ee13:	mov    QWORD PTR [rsp+0xc8],0x400
  66ee1f:	mov    QWORD PTR [rsp+0xd0],r12
  66ee27:	mov    QWORD PTR [rsp+0xd8],r14
  66ee2f:	mov    QWORD PTR [rsp+0xe0],0x400
  66ee3b:	lea    rax,[rsp+0xe8]
  66ee43:	lea    rcx,[rsp+0x80]
  66ee4b:	mov    QWORD PTR [rsp+0xe8],rcx
  66ee53:	lea    rbx,[r14+0x3ff]
  66ee5a:	shr    rbx,0xa
  66ee5e:	mov    QWORD PTR [rsp+0xf0],r13
  66ee66:	mov    QWORD PTR [rsp+0xf8],r14
  66ee6e:	mov    QWORD PTR [rsp+0x100],0x400
  66ee7a:	lea    rcx,[rsp+0xf]
  66ee7f:	mov    QWORD PTR [rsp+0x108],rcx
  66ee87:	mov    QWORD PTR [rsp+0x110],rcx
  66ee8f:	mov    QWORD PTR [rsp+0x118],rax
  66ee97:	mov    QWORD PTR [rsp+0x120],rbx
  66ee9f:	mov    QWORD PTR [rsp+0x48],r13
  66eea4:	mov    QWORD PTR [rsp+0x50],r14
  66eea9:	mov    QWORD PTR [rsp+0x58],0x400
  66eeb2:	mov    QWORD PTR [rsp+0x60],r12
  66eeb7:	mov    QWORD PTR [rsp+0x68],r14
  66eebc:	mov    QWORD PTR [rsp+0x70],0x400
  66eec5:	mov    QWORD PTR [rsp+0x78],0x0
  66eece:	mov    rax,QWORD PTR fs:[r15]
  66eed2:	test   rax,rax
  66eed5:	je     66eedf <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0x25f>
  66eed7:	add    rax,0x110
  66eedd:	jmp    66eee5 <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0x265>
  66eedf:	call   QWORD PTR [rip+0x896fb]        # 6f85e0 <_DYNAMIC+0x5c8>
  66eee5:	lea    rdx,[rsp+0x108]
  66eeed:	mov    rax,QWORD PTR [rax]
  66eef0:	mov    rcx,QWORD PTR [rax+0x208]
  66eef7:	mov    QWORD PTR [rsp],rdx
  66eefb:	lea    rdi,[rsp+0x128]
  66ef03:	lea    r9,[rsp+0x48]
  66ef08:	mov    r8d,0x1
  66ef0e:	mov    rsi,rbx
  66ef11:	xor    edx,edx
  66ef13:	call   65c5d0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_>
  66ef18:	mov    rax,QWORD PTR [rsp+0x1b8]
  66ef20:	mov    QWORD PTR [rax+0x10],r14
  66ef24:	vmovdqa ymm0,YMMWORD PTR [rip+0xffffffffff99e434]        # d360 <anon.36d21461a6996b7f2bd9a76e3225918f.28.llvm.7983859039285366149+0x1c0>
  66ef2c:	vpermq zmm1,zmm0,ZMMWORD PTR [rsp+0x128]
  66ef37:	vmovdqu ymm2,YMMWORD PTR [rsp+0x128]
  66ef40:	vmovdqu ymm3,YMMWORD PTR [rsp+0x148]
  66ef49:	mov    rax,QWORD PTR [rsp+0x1c0]
  66ef51:	mov    QWORD PTR [rax+0x10],r14
  66ef55:	vpunpcklqdq ymm4,ymm2,ymm3
  66ef59:	vextracti128 xmm4,ymm4,0x1
  66ef5f:	vpaddq xmm5,xmm4,xmm4
  66ef63:	vpunpckhqdq ymm6,ymm2,ymm3
  66ef67:	vextracti128 xmm6,ymm6,0x1
  66ef6d:	vpshldq xmm7,xmm6,xmm4,0x1
  66ef74:	vpsllq xmm8,xmm4,0x2
  66ef79:	vpshldq xmm9,xmm6,xmm4,0x2
  66ef80:	vpsllq xmm10,xmm4,0x7
  66ef85:	vpsrlvq xmm11,xmm6,XMMWORD PTR [rip+0xffffffffff99d9d2]        # c960 <anon.60695053c5df2db48177e894d4d8ab59.354.llvm.16546105846116086659+0x1a0>
  66ef8e:	vpshldq xmm4,xmm6,xmm4,0x7
  66ef95:	vpsrlvq xmm12,xmm6,XMMWORD PTR [rip+0xffffffffff99d992]        # c930 <anon.60695053c5df2db48177e894d4d8ab59.354.llvm.16546105846116086659+0x170>
  66ef9e:	vpsrlq xmm6,xmm6,0x39
  66efa3:	vpternlogq xmm6,xmm11,xmm12,0x96
  66efaa:	vinserti128 ymm11,ymm6,xmm6,0x1
  66efb0:	vmovdqa ymm12,YMMWORD PTR [rip+0xffffffffff99e708]        # d6c0 <anon.36d21461a6996b7f2bd9a76e3225918f.28.llvm.7983859039285366149+0x520>
  66efb8:	vmovdqa ymm13,ymm11
  66efbd:	vpermt2q ymm13,ymm12,ymm9
  66efc3:	vpsllvq ymm9,ymm13,YMMWORD PTR [rip+0xffffffffff99e0d4]        # d0a0 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0x420>
  66efcc:	vpsllq xmm13,xmm6,0x2
  66efd1:	vpermt2q ymm11,ymm12,ymm7
  66efd7:	vpsllvq ymm7,ymm11,YMMWORD PTR [rip+0xffffffffff99e160]        # d140 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0x4c0>
  66efe0:	vpternlogq xmm1,xmm5,xmm8,0x96
  66efe7:	vpternlogq xmm1,xmm13,xmm10,0x96
  66efee:	vperm2i128 ymm5,ymm2,ymm3,0x20
  66eff4:	vmovdqa ymm8,YMMWORD PTR [rip+0xffffffffff99e824]        # d820 <anon.36d21461a6996b7f2bd9a76e3225918f.28.llvm.7983859039285366149+0x680>
  66effc:	vpermt2q ymm1,ymm8,ymm5
  66f002:	vpternlogq ymm1,ymm7,ymm9,0x96
  66f009:	vperm2i128 ymm2,ymm2,ymm3,0x31
  66f00f:	vpermi2q ymm0,ymm2,ymm4
  66f015:	vpermi2q ymm8,ymm6,ymm2
  66f01b:	vpternlogq ymm8,ymm1,ymm0,0x96
  66f022:	mov    rbx,QWORD PTR [rsp+0x10]
  66f027:	vmovdqu YMMWORD PTR [rbx],ymm8
  66f02b:	cmp    QWORD PTR [rsp+0x30],0x0
  66f031:	je     66f041 <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0x3c1>
  66f033:	mov    rdi,QWORD PTR [rsp+0x38]
  66f038:	vzeroupper 
  66f03b:	call   QWORD PTR [rip+0x891df]        # 6f8220 <free@GLIBC_2.2.5>
  66f041:	mov    rax,rbx
  66f044:	add    rsp,0x168
  66f04b:	pop    rbx
  66f04c:	pop    r12
  66f04e:	pop    r13
  66f050:	pop    r14
  66f052:	pop    r15
  66f054:	pop    rbp
  66f055:	vzeroupper 
  66f058:	ret    
  66f059:	lea    r9,[rip+0x86248]        # 6f52a8 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x528>
  66f060:	lea    rsi,[rsp+0x18]
  66f065:	lea    rdx,[rsp+0x20]
  66f06a:	xor    edi,edi
  66f06c:	xor    ecx,ecx
  66f06e:	call   QWORD PTR [rip+0x8936c]        # 6f83e0 <_DYNAMIC+0x3c8>
  66f074:	lea    r9,[rip+0x86245]        # 6f52c0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x540>
  66f07b:	lea    rsi,[rsp+0x48]
  66f080:	lea    rdx,[rsp+0x28]
  66f085:	xor    edi,edi
  66f087:	xor    ecx,ecx
  66f089:	call   QWORD PTR [rip+0x89351]        # 6f83e0 <_DYNAMIC+0x3c8>
  66f08f:	mov    rdi,QWORD PTR [rsp+0x1b8]
  66f097:	xor    esi,esi
  66f099:	mov    rdx,r14
  66f09c:	call   6574d0 <_RINvNvMs2_NtCs3KfQW3d0ssT_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs5Palesvl9wF_6mikros>
  66f0a1:	mov    rax,QWORD PTR [rsp+0x1b8]
  66f0a9:	mov    rdx,QWORD PTR [rax]
  66f0ac:	mov    r13,QWORD PTR [rax+0x10]
  66f0b0:	sub    rdx,r13
  66f0b3:	cmp    r14,rdx
  66f0b6:	jbe    66ed16 <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0x96>
  66f0bc:	jmp    66f0eb <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0x46b>
  66f0be:	mov    rdi,QWORD PTR [rsp+0x1c0]
  66f0c6:	xor    esi,esi
  66f0c8:	mov    rdx,r14
  66f0cb:	call   6574d0 <_RINvNvMs2_NtCs3KfQW3d0ssT_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs5Palesvl9wF_6mikros>
  66f0d0:	mov    rax,QWORD PTR [rsp+0x1c0]
  66f0d8:	mov    rdx,QWORD PTR [rax]
  66f0db:	mov    r12,QWORD PTR [rax+0x10]
  66f0df:	sub    rdx,r12
  66f0e2:	cmp    r14,rdx
  66f0e5:	jbe    66ed51 <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0xd1>
  66f0eb:	lea    rcx,[rip+0x8604e]        # 6f5140 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x3c0>
  66f0f2:	xor    edi,edi
  66f0f4:	mov    rsi,r14
  66f0f7:	call   QWORD PTR [rip+0x8925b]        # 6f8358 <_DYNAMIC+0x340>
  66f0fd:	ud2    
  66f0ff:	mov    rbx,rax
  66f102:	cmp    QWORD PTR [rsp+0x30],0x0
  66f108:	je     66f115 <_RNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into+0x495>
  66f10a:	mov    rdi,QWORD PTR [rsp+0x38]
  66f10f:	call   QWORD PTR [rip+0x8910b]        # 6f8220 <free@GLIBC_2.2.5>
  66f115:	mov    rdi,rbx
  66f118:	call   6f1930 <_Unwind_Resume@plt>
  66f11d:	int3   
  66f11e:	int3   
  66f11f:	int3   

