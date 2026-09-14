00000000006638c0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_>:
  6638c0:	push   rbp
  6638c1:	push   r15
  6638c3:	push   r14
  6638c5:	push   r13
  6638c7:	push   r12
  6638c9:	push   rbx
  6638ca:	sub    rsp,0x2a8
  6638d1:	mov    DWORD PTR [rsp+0x14],ecx
  6638d5:	mov    r14,rdx
  6638d8:	mov    r12,rsi
  6638db:	mov    QWORD PTR [rsp+0x18],rdi
  6638e0:	lea    rax,[rdx+0x110]
  6638e7:	mov    rcx,QWORD PTR [rdx+0x100]
  6638ee:	mov    QWORD PTR [rsp+0x100],rax
  6638f6:	mov    QWORD PTR [rsp+0x108],0x0
  663902:	mov    QWORD PTR [rsp+0x110],rcx
  66390a:	mov    BYTE PTR [rsp+0x118],0x0
  663912:	vmovups ymm0,YMMWORD PTR [rsi]
  663916:	vmovups ymm1,YMMWORD PTR [rsi+0x20]
  66391b:	vmovups ymm2,YMMWORD PTR [rsi+0x40]
  663920:	vmovups YMMWORD PTR [rsp+0x50],ymm0
  663926:	vmovups YMMWORD PTR [rsp+0x70],ymm1
  66392c:	vmovups YMMWORD PTR [rsp+0x90],ymm2
  663935:	mov    rax,QWORD PTR [rsi+0x60]
  663939:	mov    QWORD PTR [rsp+0xb0],rax
  663941:	mov    QWORD PTR [rsp+0xb8],0x0
  66394d:	mov    rax,QWORD PTR [rdx+0x118]
  663954:	mov    rbp,QWORD PTR [rax+0x108]
  66395b:	mov    r15,QWORD PTR [rax+0x100]
  663962:	mov    rax,QWORD PTR [rdx+0x118]
  663969:	mov    rbx,QWORD PTR [rax+0x108]
  663970:	mov    rax,QWORD PTR [rax+0x100]
  663977:	mov    rsi,QWORD PTR [rdx+0x128]
  66397e:	mov    rcx,rbx
  663981:	sub    rcx,rax
  663984:	cmp    rcx,rsi
  663987:	jge    663d68 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x4a8>
  66398d:	sub    rbp,r15
  663990:	movabs rcx,0x100000000
  66399a:	mov    rax,QWORD PTR [r14+0x120]
  6639a1:	dec    rsi
  6639a4:	and    rsi,rbx
  6639a7:	shl    rsi,0x4
  6639ab:	lea    r15,[rip+0x1190e]        # 6752c0 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_>
  6639b2:	mov    QWORD PTR [rax+rsi*1],r15
  6639b6:	lea    r13,[rsp+0x50]
  6639bb:	mov    QWORD PTR [rax+rsi*1+0x8],r13
  6639c0:	mov    rax,QWORD PTR [r14+0x118]
  6639c7:	inc    rbx
  6639ca:	mov    QWORD PTR [rax+0x108],rbx
  6639d1:	mov    rsi,QWORD PTR [r14+0x110]
  6639d8:	lea    rdi,[rsi+0x1d8]
  6639df:	nop
  6639e0:	mov    rax,QWORD PTR [rsi+0x1f0]
  6639e7:	test   rax,rcx
  6639ea:	jne    663a0a <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x14a>
  6639ec:	mov    rdx,rax
  6639ef:	or     rdx,rcx
  6639f2:	lock cmpxchg QWORD PTR [rsi+0x1f0],rdx
  6639fb:	jne    6639e0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x120>
  6639fd:	mov    rax,rdx
  663a00:	and    rax,0xffff
  663a06:	jne    663a18 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x158>
  663a08:	jmp    663a2c <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x16c>
  663a0a:	mov    rdx,rax
  663a0d:	mov    rax,rdx
  663a10:	and    rax,0xffff
  663a16:	je     663a2c <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x16c>
  663a18:	test   rbp,rbp
  663a1b:	jg     663d55 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x495>
  663a21:	shr    edx,0x10
  663a24:	cmp    edx,eax
  663a26:	je     663d55 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x495>
  663a2c:	mov    rax,QWORD PTR [r12+0x68]
  663a31:	mov    rdx,QWORD PTR [r12+0x70]
  663a36:	vmovups ymm0,YMMWORD PTR [r12+0xa8]
  663a40:	vmovups YMMWORD PTR [rsp+0x1c0],ymm0
  663a49:	vmovups ymm0,YMMWORD PTR [r12+0x78]
  663a50:	vmovups ymm1,YMMWORD PTR [r12+0x88]
  663a5a:	vmovups YMMWORD PTR [rsp+0x190],ymm0
  663a63:	vmovups YMMWORD PTR [rsp+0x1a0],ymm1
  663a6c:	mov    rsi,QWORD PTR [rax]
  663a6f:	mov    rcx,QWORD PTR [rdx]
  663a72:	mov    r8,QWORD PTR [rdx+0x8]
  663a76:	add    r12,0xb0
  663a7d:	mov    QWORD PTR [rsp],r12
  663a81:	movzx  ebp,BYTE PTR [rsp+0x14]
  663a86:	lea    rdi,[rsp+0x150]
  663a8e:	lea    r9,[rsp+0x190]
  663a96:	mov    edx,ebp
  663a98:	vzeroupper 
  663a9b:	call   65ca30 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_>
  663aa0:	mov    r12,QWORD PTR [rsp+0x150]
  663aa8:	mov    rbx,QWORD PTR [rsp+0x158]
  663ab0:	vmovups ymm0,YMMWORD PTR [rsp+0x170]
  663ab9:	vmovups YMMWORD PTR [rsp+0x30],ymm0
  663abf:	vmovups ymm0,YMMWORD PTR [rsp+0x160]
  663ac8:	vmovups YMMWORD PTR [rsp+0x20],ymm0
  663ace:	vmovups ymm0,YMMWORD PTR [rsp+0x20]
  663ad4:	vmovups ymm1,YMMWORD PTR [rsp+0x30]
  663ada:	vmovups YMMWORD PTR [rsp+0x120],ymm0
  663ae3:	vmovups YMMWORD PTR [rsp+0x130],ymm1
  663aec:	nop    DWORD PTR [rax+0x0]
  663af0:	mov    rax,QWORD PTR [rsp+0x108]
  663af8:	cmp    rax,0x3
  663afc:	je     663b3c <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x27c>
  663afe:	mov    rdi,r14
  663b01:	vzeroupper 
  663b04:	call   66b360 <_RNvMs8_NtCsiKyBw8IgE1b_10rayon_core8registryNtB5_12WorkerThread14take_local_job>
  663b09:	test   rax,rax
  663b0c:	je     663b2a <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x26a>
  663b0e:	mov    rcx,r13
  663b11:	xor    rcx,rdx
  663b14:	mov    rsi,rax
  663b17:	xor    rsi,r15
  663b1a:	or     rsi,rcx
  663b1d:	je     663bbf <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x2ff>
  663b23:	mov    rdi,rdx
  663b26:	call   rax
  663b28:	jmp    663af0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x230>
  663b2a:	mov    rax,QWORD PTR [rsp+0x108]
  663b32:	cmp    rax,0x3
  663b36:	jne    663da2 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x4e2>
  663b3c:	mov    rax,QWORD PTR [rsp+0xb8]
  663b44:	vmovdqa xmm0,XMMWORD PTR [rsp+0xc0]
  663b4d:	cmp    rax,0x1
  663b51:	je     663b71 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x2b1>
  663b53:	cmp    rax,0x2
  663b57:	jne    663d86 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x4c6>
  663b5d:	vmovq  rdi,xmm0
  663b62:	vpextrq rsi,xmm0,0x1
  663b68:	vzeroupper 
  663b6b:	call   QWORD PTR [rip+0x94a87]        # 6f85f8 <_DYNAMIC+0x5e0>
  663b71:	vmovups ymm1,YMMWORD PTR [rsp+0xd0]
  663b7a:	vmovups ymm2,YMMWORD PTR [rsp+0xe0]
  663b83:	mov    rax,QWORD PTR [rsp+0x18]
  663b88:	vmovups YMMWORD PTR [rax+0x60],ymm2
  663b8d:	vmovups YMMWORD PTR [rax+0x50],ymm1
  663b92:	mov    QWORD PTR [rax],r12
  663b95:	mov    QWORD PTR [rax+0x8],rbx
  663b99:	vmovups ymm1,YMMWORD PTR [rsp+0x120]
  663ba2:	vmovups ymm2,YMMWORD PTR [rsp+0x130]
  663bab:	vmovups YMMWORD PTR [rax+0x10],ymm1
  663bb0:	vmovups YMMWORD PTR [rax+0x20],ymm2
  663bb5:	vmovdqu XMMWORD PTR [rax+0x40],xmm0
  663bba:	jmp    663d40 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x480>
  663bbf:	vmovups ymm0,YMMWORD PTR [rsp+0x50]
  663bc5:	vmovups ymm1,YMMWORD PTR [rsp+0x70]
  663bcb:	vmovups ymm2,YMMWORD PTR [rsp+0x90]
  663bd4:	vmovups ymm3,YMMWORD PTR [rsp+0xb0]
  663bdd:	vmovups YMMWORD PTR [rsp+0x190],ymm0
  663be6:	vmovups ymm0,YMMWORD PTR [rsp+0x100]
  663bef:	vmovups YMMWORD PTR [rsp+0x240],ymm0
  663bf8:	vmovups ymm0,YMMWORD PTR [rsp+0xf0]
  663c01:	vmovups YMMWORD PTR [rsp+0x230],ymm0
  663c0a:	vmovups ymm0,YMMWORD PTR [rsp+0xd0]
  663c13:	vmovups YMMWORD PTR [rsp+0x210],ymm0
  663c1c:	vmovups YMMWORD PTR [rsp+0x1f0],ymm3
  663c25:	vmovups YMMWORD PTR [rsp+0x1d0],ymm2
  663c2e:	vmovups YMMWORD PTR [rsp+0x1b0],ymm1
  663c37:	mov    rax,QWORD PTR [rsp+0x190]
  663c3f:	test   rax,rax
  663c42:	je     663db8 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x4f8>
  663c48:	mov    rcx,QWORD PTR [rsp+0x198]
  663c50:	vmovups ymm0,YMMWORD PTR [rsp+0x68]
  663c56:	vmovups ymm1,YMMWORD PTR [rsp+0x80]
  663c5f:	vmovups YMMWORD PTR [rsp+0x168],ymm1
  663c68:	vmovups YMMWORD PTR [rsp+0x150],ymm0
  663c71:	mov    rdx,QWORD PTR [rsp+0x1a0]
  663c79:	mov    rsi,QWORD PTR [rsp+0xb0]
  663c81:	vmovaps xmm0,XMMWORD PTR [rsp+0xa0]
  663c8a:	vmovaps XMMWORD PTR [rsp+0x20],xmm0
  663c90:	mov    QWORD PTR [rsp+0x30],rsi
  663c95:	mov    rsi,QWORD PTR [rax]
  663c98:	sub    rsi,QWORD PTR [rcx]
  663c9b:	mov    rcx,QWORD PTR [rdx]
  663c9e:	mov    r8,QWORD PTR [rdx+0x8]
  663ca2:	lea    rax,[rsp+0x20]
  663ca7:	mov    QWORD PTR [rsp],rax
  663cab:	lea    rdi,[rsp+0x268]
  663cb3:	lea    r9,[rsp+0x150]
  663cbb:	mov    edx,ebp
  663cbd:	vzeroupper 
  663cc0:	call   65ca30 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_>
  663cc5:	cmp    DWORD PTR [rsp+0x1f8],0x2
  663ccd:	jb     663cfc <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x43c>
  663ccf:	mov    r14,QWORD PTR [rsp+0x200]
  663cd7:	mov    r15,QWORD PTR [rsp+0x208]
  663cdf:	mov    rax,QWORD PTR [r15]
  663ce2:	test   rax,rax
  663ce5:	je     663cec <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x42c>
  663ce7:	mov    rdi,r14
  663cea:	call   rax
  663cec:	cmp    QWORD PTR [r15+0x8],0x0
  663cf1:	je     663cfc <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x43c>
  663cf3:	mov    rdi,r14
  663cf6:	call   QWORD PTR [rip+0x94524]        # 6f8220 <free@GLIBC_2.2.5>
  663cfc:	vmovups ymm0,YMMWORD PTR [rsp+0x120]
  663d05:	vmovups ymm1,YMMWORD PTR [rsp+0x130]
  663d0e:	mov    rax,QWORD PTR [rsp+0x18]
  663d13:	vmovups YMMWORD PTR [rax+0x20],ymm1
  663d18:	vmovups YMMWORD PTR [rax+0x10],ymm0
  663d1d:	mov    QWORD PTR [rax],r12
  663d20:	mov    QWORD PTR [rax+0x8],rbx
  663d24:	vmovdqu ymm0,YMMWORD PTR [rsp+0x268]
  663d2d:	vmovups ymm1,YMMWORD PTR [rsp+0x288]
  663d36:	vmovdqu YMMWORD PTR [rax+0x40],ymm0
  663d3b:	vmovups YMMWORD PTR [rax+0x60],ymm1
  663d40:	add    rsp,0x2a8
  663d47:	pop    rbx
  663d48:	pop    r12
  663d4a:	pop    r13
  663d4c:	pop    r14
  663d4e:	pop    r15
  663d50:	pop    rbp
  663d51:	vzeroupper 
  663d54:	ret    
  663d55:	mov    esi,0x1
  663d5a:	vzeroupper 
  663d5d:	call   QWORD PTR [rip+0x9489d]        # 6f8600 <_DYNAMIC+0x5e8>
  663d63:	jmp    663a2c <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x16c>
  663d68:	lea    rdi,[r14+0x118]
  663d6f:	add    rsi,rsi
  663d72:	vzeroupper 
  663d75:	call   66af10 <_RNvMs4_NtCsh4KpTWVPfp2_15crossbeam_deque5dequeINtB5_6WorkerNtNtCsiKyBw8IgE1b_10rayon_core3job6JobRefE6resizeCs5Palesvl9wF_6mikros>
  663d7a:	mov    rsi,QWORD PTR [r14+0x128]
  663d81:	jmp    66398d <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0xcd>
  663d86:	lea    rdi,[rip+0xffffffffff9ae8cf]        # 1265c <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x160e>
  663d8d:	lea    rdx,[rip+0x9134c]        # 6f50e0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x360>
  663d94:	mov    esi,0x28
  663d99:	vzeroupper 
  663d9c:	call   QWORD PTR [rip+0x945be]        # 6f8360 <_DYNAMIC+0x348>
  663da2:	mov    rdi,r14
  663da5:	lea    rsi,[rsp+0x108]
  663dad:	call   QWORD PTR [rip+0x94855]        # 6f8608 <_DYNAMIC+0x5f0>
  663db3:	jmp    663b3c <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x27c>
  663db8:	lea    rdi,[rip+0x912f1]        # 6f50b0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x330>
  663dbf:	vzeroupper 
  663dc2:	call   QWORD PTR [rip+0x945f0]        # 6f83b8 <_DYNAMIC+0x3a0>
  663dc8:	jmp    663e07 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x547>
  663dca:	jmp    663e0b <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x54b>
  663dcc:	mov    rbx,rax
  663dcf:	cmp    QWORD PTR [r15+0x8],0x0
  663dd4:	je     663e30 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x570>
  663dd6:	mov    rdi,r14
  663dd9:	call   QWORD PTR [rip+0x94441]        # 6f8220 <free@GLIBC_2.2.5>
  663ddf:	mov    rdi,rbx
  663de2:	call   6f1930 <_Unwind_Resume@plt>
  663de7:	mov    rdi,rax
  663dea:	call   QWORD PTR [rip+0x94868]        # 6f8658 <_DYNAMIC+0x640>
  663df0:	mov    rcx,rdx
  663df3:	mov    rdi,r14
  663df6:	lea    rsi,[rsp+0x100]
  663dfe:	mov    rdx,rax
  663e01:	call   QWORD PTR [rip+0x94859]        # 6f8660 <_DYNAMIC+0x648>
  663e07:	ud2    
  663e09:	jmp    663e0b <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x54b>
  663e0b:	mov    rbx,rax
  663e0e:	lea    rdi,[rsp+0x50]
  663e13:	call   656e50 <_RINvNtCsk9TLBDxQNFU_4core3ptr13drop_in_placeINtNtCsiKyBw8IgE1b_10rayon_core3job8StackJobNtNtBL_5latch9SpinLatchNCINvNvNtBL_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3s_9enumerate17EnumerateProducerINtNtB3s_3zip11ZipProducerINtNtNtB3u_5slice6chunks17ChunksMutProducerINtNtNtB4_3mem12maybe_uninit11MaybeUninitNtB2q_4F128EEB5B_EEINtNtB3s_3map11MapConsumerINtNtB3s_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8e_s0_0ENCB8e_s_0EEs_0E0B2n_EEB8i_>
  663e18:	jmp    663e30 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_+0x570>
  663e1a:	call   QWORD PTR [rip+0x946a8]        # 6f84c8 <_DYNAMIC+0x4b0>
  663e20:	mov    rbx,rax
  663e23:	lea    rdi,[rsp+0x1f8]
  663e2b:	call   656760 <_RINvNtCsk9TLBDxQNFU_4core3ptr13drop_in_placeINtNtB4_4cell10UnsafeCellINtNtCsiKyBw8IgE1b_10rayon_core3job9JobResultANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_EEECs5Palesvl9wF_6mikros>
  663e30:	mov    rdi,rbx
  663e33:	call   6f1930 <_Unwind_Resume@plt>
  663e38:	call   QWORD PTR [rip+0x9468a]        # 6f84c8 <_DYNAMIC+0x4b0>
  663e3e:	int3   
  663e3f:	int3   

