0000000000675040 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_>:
  675040:	push   rbp
  675041:	push   r15
  675043:	push   r14
  675045:	push   r13
  675047:	push   r12
  675049:	push   rbx
  67504a:	sub    rsp,0xe8
  675051:	mov    rcx,QWORD PTR [rdi]
  675054:	mov    rax,QWORD PTR [rdi+0x8]
  675058:	mov    QWORD PTR [rdi],0x0
  67505f:	test   rcx,rcx
  675062:	je     67520f <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x1cf>
  675068:	mov    rbx,rdi
  67506b:	mov    rdx,QWORD PTR [rdi+0x60]
  67506f:	mov    QWORD PTR [rsp+0xa0],rdx
  675077:	vmovups ymm0,YMMWORD PTR [rdi+0x40]
  67507c:	vmovups YMMWORD PTR [rsp+0x80],ymm0
  675085:	vmovups ymm0,YMMWORD PTR [rdi+0x10]
  67508a:	vmovups ymm1,YMMWORD PTR [rdi+0x20]
  67508f:	vmovups YMMWORD PTR [rsp+0x50],ymm0
  675095:	vmovups YMMWORD PTR [rsp+0x60],ymm1
  67509b:	mov    QWORD PTR [rsp+0x40],rcx
  6750a0:	mov    QWORD PTR [rsp+0x48],rax
  6750a5:	mov    rdx,QWORD PTR [rsp+0x50]
  6750aa:	lea    r9,[rsp+0x58]
  6750af:	lea    rdi,[rsp+0x90]
  6750b7:	mov    rsi,QWORD PTR [rcx]
  6750ba:	sub    rsi,QWORD PTR [rax]
  6750bd:	mov    rcx,QWORD PTR [rdx]
  6750c0:	mov    r8,QWORD PTR [rdx+0x8]
  6750c4:	mov    QWORD PTR [rsp],rdi
  6750c8:	lea    rdi,[rsp+0xa8]
  6750d0:	mov    edx,0x1
  6750d5:	vzeroupper 
  6750d8:	call   65c5d0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_>
  6750dd:	mov    r14,QWORD PTR [rsp+0xa8]
  6750e5:	vmovups ymm0,YMMWORD PTR [rsp+0xc8]
  6750ee:	vmovups YMMWORD PTR [rsp+0x50],ymm0
  6750f4:	vmovups ymm0,YMMWORD PTR [rsp+0xb8]
  6750fd:	vmovups YMMWORD PTR [rsp+0x40],ymm0
  675103:	vmovups ymm0,YMMWORD PTR [rsp+0x40]
  675109:	vmovups ymm1,YMMWORD PTR [rsp+0x50]
  67510f:	mov    r15,QWORD PTR [rsp+0xb0]
  675117:	vmovups YMMWORD PTR [rsp+0x10],ymm0
  67511d:	vmovups YMMWORD PTR [rsp+0x20],ymm1
  675123:	mov    r13d,0x1
  675129:	cmp    DWORD PTR [rbx+0x68],0x2
  67512d:	jb     67515b <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x11b>
  67512f:	mov    r12,QWORD PTR [rbx+0x70]
  675133:	mov    rbp,QWORD PTR [rbx+0x78]
  675137:	mov    rax,QWORD PTR [rbp+0x0]
  67513b:	test   rax,rax
  67513e:	je     675148 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x108>
  675140:	mov    rdi,r12
  675143:	vzeroupper 
  675146:	call   rax
  675148:	cmp    QWORD PTR [rbp+0x8],0x0
  67514d:	je     67515b <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x11b>
  67514f:	mov    rdi,r12
  675152:	vzeroupper 
  675155:	call   QWORD PTR [rip+0x830c5]        # 6f8220 <free@GLIBC_2.2.5>
  67515b:	mov    QWORD PTR [rbx+0x68],r13
  67515f:	mov    QWORD PTR [rbx+0x70],r14
  675163:	mov    QWORD PTR [rbx+0x78],r15
  675167:	vmovups ymm0,YMMWORD PTR [rsp+0x10]
  67516d:	vmovups ymm1,YMMWORD PTR [rsp+0x20]
  675173:	vmovups YMMWORD PTR [rbx+0x80],ymm0
  67517b:	vmovups YMMWORD PTR [rbx+0x90],ymm1
  675183:	movzx  ebp,BYTE PTR [rbx+0xc8]
  67518a:	test   bpl,bpl
  67518d:	je     6751aa <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x16a>
  67518f:	mov    rax,QWORD PTR [rbx+0xb0]
  675196:	mov    rdi,QWORD PTR [rax]
  675199:	lock inc QWORD PTR [rdi]
  67519d:	jle    6752b0 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x270>
  6751a3:	mov    QWORD PTR [rsp+0x10],rdi
  6751a8:	jmp    6751b4 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x174>
  6751aa:	mov    rax,QWORD PTR [rbx+0xb0]
  6751b1:	mov    rdi,QWORD PTR [rax]
  6751b4:	mov    rsi,QWORD PTR [rbx+0xc0]
  6751bb:	mov    eax,0x3
  6751c0:	xchg   QWORD PTR [rbx+0xb8],rax
  6751c7:	cmp    rax,0x2
  6751cb:	jne    6751dc <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x19c>
  6751cd:	add    rdi,0x1d8
  6751d4:	vzeroupper 
  6751d7:	call   67c660 <_RNvMNtCsiKyBw8IgE1b_10rayon_core5sleepNtB2_5Sleep20wake_specific_thread.llvm.10825995516073413387>
  6751dc:	test   bpl,bpl
  6751df:	je     6751fa <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x1ba>
  6751e1:	mov    rax,QWORD PTR [rsp+0x10]
  6751e6:	lock dec QWORD PTR [rax]
  6751ea:	jne    6751fa <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x1ba>
  6751ec:	lea    rdi,[rsp+0x10]
  6751f1:	vzeroupper 
  6751f4:	call   QWORD PTR [rip+0x8359e]        # 6f8798 <_DYNAMIC+0x780>
  6751fa:	add    rsp,0xe8
  675201:	pop    rbx
  675202:	pop    r12
  675204:	pop    r13
  675206:	pop    r14
  675208:	pop    r15
  67520a:	pop    rbp
  67520b:	vzeroupper 
  67520e:	ret    
  67520f:	lea    rdi,[rip+0x803f2]        # 6f5608 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x888>
  675216:	call   QWORD PTR [rip+0x8319c]        # 6f83b8 <_DYNAMIC+0x3a0>
  67521c:	jmp    6752b0 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x270>
  675221:	cmp    QWORD PTR [rbp+0x8],0x0
  675226:	je     675231 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x1f1>
  675228:	mov    rdi,r12
  67522b:	call   QWORD PTR [rip+0x82fef]        # 6f8220 <free@GLIBC_2.2.5>
  675231:	mov    QWORD PTR [rbx+0x68],r13
  675235:	mov    QWORD PTR [rbx+0x70],r14
  675239:	mov    QWORD PTR [rbx+0x78],r15
  67523d:	vmovups ymm0,YMMWORD PTR [rsp+0x10]
  675243:	vmovups ymm1,YMMWORD PTR [rsp+0x20]
  675249:	vmovups YMMWORD PTR [rbx+0x80],ymm0
  675251:	vmovups YMMWORD PTR [rbx+0x90],ymm1
  675259:	jmp    6752a2 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x262>
  67525b:	test   bpl,bpl
  67525e:	je     6752a2 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x262>
  675260:	mov    rax,QWORD PTR [rsp+0x10]
  675265:	lock dec QWORD PTR [rax]
  675269:	jne    6752a2 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x262>
  67526b:	lea    rdi,[rsp+0x10]
  675270:	call   QWORD PTR [rip+0x83522]        # 6f8798 <_DYNAMIC+0x780>
  675276:	jmp    6752a2 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x262>
  675278:	call   QWORD PTR [rip+0x8324a]        # 6f84c8 <_DYNAMIC+0x4b0>
  67527e:	mov    rdi,rax
  675281:	call   QWORD PTR [rip+0x833d1]        # 6f8658 <_DYNAMIC+0x640>
  675287:	mov    r14,rax
  67528a:	mov    r15,rdx
  67528d:	mov    r13d,0x2
  675293:	cmp    DWORD PTR [rbx+0x68],0x2
  675297:	jae    67512f <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0xef>
  67529d:	jmp    67515b <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x11b>
  6752a2:	lea    rdi,[rsp+0xf]
  6752a7:	vzeroupper 
  6752aa:	call   QWORD PTR [rip+0x834e0]        # 6f8790 <_DYNAMIC+0x778>
  6752b0:	ud2    
  6752b2:	call   QWORD PTR [rip+0x83210]        # 6f84c8 <_DYNAMIC+0x4b0>
  6752b8:	int3   
  6752b9:	int3   
  6752ba:	int3   
  6752bb:	int3   
  6752bc:	int3   
  6752bd:	int3   
  6752be:	int3   
  6752bf:	int3   

