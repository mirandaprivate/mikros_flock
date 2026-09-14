00000000006650d0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_>:
  6650d0:	push   rbp
  6650d1:	push   r15
  6650d3:	push   r14
  6650d5:	push   r13
  6650d7:	push   r12
  6650d9:	push   rbx
  6650da:	sub    rsp,0x218
  6650e1:	mov    QWORD PTR [rsp+0x48],r9
  6650e6:	mov    QWORD PTR [rsp+0x40],rcx
  6650eb:	mov    QWORD PTR [rsp+0xb0],rdi
  6650f3:	lea    r10,[r8+rdx*1]
  6650f7:	mov    rbx,QWORD PTR [rsi+0x10]
  6650fb:	mov    rax,rdx
  6650fe:	shl    rax,0x4
  665102:	mov    r11,r10
  665105:	shl    r11,0x4
  665109:	mov    QWORD PTR [rsp],rax
  66510d:	cmp    r11,rax
  665110:	jb     666bcb <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1afb>
  665116:	cmp    r11,rbx
  665119:	ja     666bcb <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1afb>
  66511f:	mov    rbx,QWORD PTR [rsi+0x20]
  665123:	cmp    r11,rbx
  665126:	ja     666bf8 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1b28>
  66512c:	mov    r11,QWORD PTR [rsi+0x30]
  665130:	cmp    r10,rdx
  665133:	jb     666be2 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1b12>
  665139:	cmp    r10,r11
  66513c:	ja     666be2 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1b12>
  665142:	mov    rax,QWORD PTR [rsp+0x250]
  66514a:	mov    r9,QWORD PTR [rsi]
  66514d:	mov    r11,QWORD PTR [rsi+0x8]
  665151:	mov    rdi,QWORD PTR [rsi+0x18]
  665155:	mov    rcx,QWORD PTR [rsi+0x28]
  665159:	mov    rbx,QWORD PTR [rsp]
  66515d:	add    rcx,rbx
  665160:	mov    QWORD PTR [rsp+0x38],rcx
  665165:	mov    QWORD PTR [rsp+0xc8],rax
  66516d:	mov    QWORD PTR [rsp+0x190],r8
  665175:	cmp    rax,r8
  665178:	jne    666c0f <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1b3f>
  66517e:	lea    rax,[rdi+rbx*1]
  665182:	mov    QWORD PTR [rsp+0xc0],rax
  66518a:	add    r11,rbx
  66518d:	mov    rdx,r8
  665190:	shl    rdx,0x4
  665194:	mov    rsi,r8
  665197:	shr    rsi,0x3
  66519b:	mov    QWORD PTR [rsp+0x30],r8
  6651a0:	je     666430 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1360>
  6651a6:	mov    r13,QWORD PTR [r9+0x8]
  6651aa:	mov    eax,0x33
  6651af:	mov    QWORD PTR [rsp+0x18],r9
  6651b4:	bzhi   rbp,QWORD PTR [r9+0x10],rax
  6651ba:	shr    rbp,0x2
  6651be:	cmp    rbp,0x4
  6651c2:	mov    r14d,0x4
  6651c8:	cmovb  r14,rbp
  6651cc:	mov    QWORD PTR [rsp+0x28],r14
  6651d1:	and    r14d,0x6
  6651d5:	vpxor  xmm0,xmm0,xmm0
  6651d9:	vpxor  xmm4,xmm4,xmm4
  6651dd:	vpxor  xmm3,xmm3,xmm3
  6651e1:	vpxor  xmm2,xmm2,xmm2
  6651e5:	vpxor  xmm1,xmm1,xmm1
  6651e9:	vpxord xmm27,xmm27,xmm27
  6651ef:	mov    QWORD PTR [rsp+0x8],rdi
  6651f4:	lea    rax,[rbx+rdi*1+0x7]
  6651f9:	mov    QWORD PTR [rsp+0x10],rax
  6651fe:	xor    r15d,r15d
  665201:	jmp    6653ac <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x2dc>
  665206:	cs nop WORD PTR [rax+rax*1+0x0]
  665210:	vmovdqu64 zmm9,ZMMWORD PTR [rsp+0x190]
  66521b:	vmovdqu64 zmm8,ZMMWORD PTR [rsp+0x1d0]
  665226:	mov    rsi,QWORD PTR [rsp+0x20]
  66522b:	lea    rax,[r15+0x8]
  66522f:	dec    rsi
  665232:	shl    r15,0x4
  665236:	mov    rcx,QWORD PTR [rsp+0x38]
  66523b:	vmovdqu64 zmm10,ZMMWORD PTR [rcx+r15*1]
  665242:	vmovdqu64 zmm11,ZMMWORD PTR [rcx+r15*1+0x40]
  66524a:	vpclmulhqlqdq zmm12,zmm9,zmm10
  665251:	vpclmullqhqdq zmm13,zmm9,zmm10
  665258:	vpxorq zmm12,zmm13,zmm12
  66525e:	vpclmulhqhqdq zmm13,zmm9,zmm10
  665265:	vpslldq zmm14,zmm13,0x8
  66526c:	vpbroadcastq zmm15,QWORD PTR [rip+0xffffffffff9a795a]        # cbd0 <anon.36d21461a6996b7f2bd9a76e3225918f.29.llvm.7983859039285366149+0xa0>
  665276:	vpclmulhqlqdq zmm13,zmm13,zmm15
  66527d:	vpternlogq zmm13,zmm12,zmm14,0x96
  665284:	vpclmullqlqdq zmm9,zmm9,zmm10
  66528b:	vpslldq zmm10,zmm13,0x8
  665292:	vpclmulhqlqdq zmm12,zmm13,zmm15
  665299:	vpternlogq zmm12,zmm10,zmm9,0x96
  6652a0:	vpclmulhqlqdq zmm9,zmm8,zmm11
  6652a7:	vpclmullqhqdq zmm10,zmm8,zmm11
  6652ae:	vpxorq zmm9,zmm10,zmm9
  6652b4:	vpclmulhqhqdq zmm10,zmm8,zmm11
  6652bb:	vpslldq zmm13,zmm10,0x8
  6652c2:	vpclmulhqlqdq zmm10,zmm10,zmm15
  6652c9:	vpternlogq zmm10,zmm9,zmm13,0x96
  6652d0:	vpclmullqlqdq zmm8,zmm8,zmm11
  6652d7:	vpslldq zmm9,zmm10,0x8
  6652de:	vpclmulhqlqdq zmm10,zmm10,zmm15
  6652e5:	vpternlogq zmm10,zmm9,zmm8,0x96
  6652ec:	vshufi64x2 zmm8,zmm7,zmm6,0x88
  6652f3:	vshufi64x2 zmm9,zmm7,zmm6,0xdd
  6652fa:	vshufi64x2 zmm11,zmm12,zmm10,0x88
  665301:	vshufi64x2 zmm13,zmm12,zmm10,0xdd
  665308:	vpclmullqlqdq zmm14,zmm8,zmm11
  66530f:	vpxorq zmm0,zmm14,zmm0
  665315:	vpclmulhqhqdq zmm14,zmm8,zmm11
  66531c:	vpxorq zmm4,zmm14,zmm4
  665322:	vpclmulhqlqdq zmm14,zmm8,zmm11
  665329:	vpclmullqhqdq zmm15,zmm8,zmm11
  665330:	vpternlogq zmm3,zmm14,zmm15,0x96
  665337:	vpxorq zmm8,zmm9,zmm8
  66533d:	vpxorq zmm9,zmm13,zmm11
  665343:	vpclmullqlqdq zmm11,zmm8,zmm9
  66534a:	vpxorq zmm2,zmm11,zmm2
  665350:	vpclmulhqhqdq zmm11,zmm8,zmm9
  665357:	vpxorq zmm1,zmm11,zmm1
  66535d:	vpclmulhqlqdq zmm11,zmm8,zmm9
  665364:	vpclmullqhqdq zmm8,zmm8,zmm9
  66536b:	vpternlogq zmm27,zmm11,zmm8,0x96
  665372:	mov    rcx,QWORD PTR [rsp+0x40]
  665377:	vmovdqu64 ZMMWORD PTR [rcx+r15*1],zmm7
  66537e:	vmovdqu64 ZMMWORD PTR [rcx+r15*1+0x40],zmm6
  665386:	mov    rcx,QWORD PTR [rsp+0x48]
  66538b:	vmovdqu64 ZMMWORD PTR [rcx+r15*1],zmm12
  665392:	vmovdqu64 ZMMWORD PTR [rcx+r15*1+0x40],zmm10
  66539a:	sub    QWORD PTR [rsp+0x10],0xffffffffffffff80
  6653a0:	mov    r15,rax
  6653a3:	test   rsi,rsi
  6653a6:	je     66644c <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x137c>
  6653ac:	mov    r10,r15
  6653af:	shl    r10,0x4
  6653b3:	lea    rax,[r10+0xf]
  6653b7:	test   rbp,rbp
  6653ba:	je     6663a0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x12d0>
  6653c0:	cmp    rax,rdx
  6653c3:	jae    666bb4 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ae4>
  6653c9:	mov    QWORD PTR [rsp+0x20],rsi
  6653ce:	movzx  eax,BYTE PTR [r11+r10*1]
  6653d3:	shl    eax,0x4
  6653d6:	movzx  esi,BYTE PTR [r11+r10*1+0x1]
  6653dc:	shl    esi,0x4
  6653df:	movzx  r8d,BYTE PTR [r11+r10*1+0x2]
  6653e5:	shl    r8d,0x4
  6653e9:	movzx  r9d,BYTE PTR [r11+r10*1+0x3]
  6653ef:	vmovdqa xmm15,XMMWORD PTR [r13+rax*1+0x0]
  6653f6:	lea    rax,[r13+0x1000]
  6653fd:	vmovdqa64 xmm16,XMMWORD PTR [rax+rsi*1]
  665404:	shl    r9d,0x4
  665408:	lea    rax,[r13+0x2000]
  66540f:	vmovdqa64 xmm17,XMMWORD PTR [rax+r8*1]
  665416:	lea    rax,[r13+0x3000]
  66541d:	vmovdqa xmm7,XMMWORD PTR [rax+r9*1]
  665423:	cmp    rbp,0x1
  665427:	je     665550 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x480>
  66542d:	movzx  eax,BYTE PTR [r11+r10*1+0x4]
  665433:	shl    eax,0x4
  665436:	movzx  ecx,BYTE PTR [r11+r10*1+0x5]
  66543c:	shl    ecx,0x4
  66543f:	movzx  esi,BYTE PTR [r11+r10*1+0x6]
  665445:	shl    esi,0x4
  665448:	movzx  r8d,BYTE PTR [r11+r10*1+0x7]
  66544e:	lea    rdi,[r13+0x4000]
  665455:	vpxor  xmm15,xmm15,XMMWORD PTR [rdi+rax*1]
  66545a:	lea    rax,[r13+0x5000]
  665461:	vpxorq xmm16,xmm16,XMMWORD PTR [rax+rcx*1]
  665468:	shl    r8d,0x4
  66546c:	lea    rax,[r13+0x6000]
  665473:	vpxorq xmm17,xmm17,XMMWORD PTR [rax+rsi*1]
  66547a:	lea    rax,[r13+0x7000]
  665481:	vpxor  xmm7,xmm7,XMMWORD PTR [rax+r8*1]
  665487:	cmp    rbp,0x2
  66548b:	je     665550 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x480>
  665491:	movzx  eax,BYTE PTR [r11+r10*1+0x8]
  665497:	shl    eax,0x4
  66549a:	movzx  ecx,BYTE PTR [r11+r10*1+0x9]
  6654a0:	shl    ecx,0x4
  6654a3:	movzx  esi,BYTE PTR [r11+r10*1+0xa]
  6654a9:	shl    esi,0x4
  6654ac:	movzx  r8d,BYTE PTR [r11+r10*1+0xb]
  6654b2:	lea    rdi,[r13+0x8000]
  6654b9:	vpxor  xmm15,xmm15,XMMWORD PTR [rdi+rax*1]
  6654be:	lea    rax,[r13+0x9000]
  6654c5:	vpxorq xmm16,xmm16,XMMWORD PTR [rax+rcx*1]
  6654cc:	shl    r8d,0x4
  6654d0:	lea    rax,[r13+0xa000]
  6654d7:	vpxorq xmm17,xmm17,XMMWORD PTR [rax+rsi*1]
  6654de:	lea    rax,[r13+0xb000]
  6654e5:	vpxor  xmm7,xmm7,XMMWORD PTR [rax+r8*1]
  6654eb:	cmp    rbp,0x3
  6654ef:	je     665550 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x480>
  6654f1:	movzx  eax,BYTE PTR [r11+r10*1+0xc]
  6654f7:	shl    eax,0x4
  6654fa:	movzx  ecx,BYTE PTR [r11+r10*1+0xd]
  665500:	shl    ecx,0x4
  665503:	movzx  esi,BYTE PTR [r11+r10*1+0xe]
  665509:	shl    esi,0x4
  66550c:	movzx  r8d,BYTE PTR [r11+r10*1+0xf]
  665512:	shl    r8d,0x4
  665516:	lea    rdi,[r13+0xc000]
  66551d:	vpxor  xmm15,xmm15,XMMWORD PTR [rdi+rax*1]
  665522:	lea    rax,[r13+0xd000]
  665529:	vpxorq xmm16,xmm16,XMMWORD PTR [rax+rcx*1]
  665530:	lea    rax,[r13+0xe000]
  665537:	vpxorq xmm17,xmm17,XMMWORD PTR [rax+rsi*1]
  66553e:	lea    rax,[r13+0xf000]
  665545:	vpxor  xmm7,xmm7,XMMWORD PTR [rax+r8*1]
  66554b:	nop    DWORD PTR [rax+rax*1+0x0]
  665550:	mov    rax,r15
  665553:	shl    rax,0x4
  665557:	mov    r10,rax
  66555a:	or     r10,0x10
  66555e:	mov    rsi,rax
  665561:	or     rsi,0x1f
  665565:	cmp    rsi,rdx
  665568:	jae    666bb4 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ae4>
  66556e:	movzx  esi,BYTE PTR [r11+r10*1]
  665573:	shl    esi,0x4
  665576:	movzx  r8d,BYTE PTR [r11+r10*1+0x1]
  66557c:	shl    r8d,0x4
  665580:	movzx  r9d,BYTE PTR [r11+r10*1+0x2]
  665586:	shl    r9d,0x4
  66558a:	movzx  ecx,BYTE PTR [r11+r10*1+0x3]
  665590:	vmovdqa64 xmm19,XMMWORD PTR [r13+rsi*1+0x0]
  665598:	lea    rsi,[r13+0x1000]
  66559f:	vmovdqa64 xmm20,XMMWORD PTR [rsi+r8*1]
  6655a6:	shl    ecx,0x4
  6655a9:	lea    rsi,[r13+0x2000]
  6655b0:	vmovdqa64 xmm21,XMMWORD PTR [rsi+r9*1]
  6655b7:	lea    rsi,[r13+0x3000]
  6655be:	vmovdqa xmm14,XMMWORD PTR [rsi+rcx*1]
  6655c3:	cmp    rbp,0x1
  6655c7:	je     665700 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x630>
  6655cd:	movzx  ecx,BYTE PTR [r11+r10*1+0x4]
  6655d3:	shl    ecx,0x4
  6655d6:	movzx  esi,BYTE PTR [r11+r10*1+0x5]
  6655dc:	shl    esi,0x4
  6655df:	movzx  r8d,BYTE PTR [r11+r10*1+0x6]
  6655e5:	shl    r8d,0x4
  6655e9:	movzx  r9d,BYTE PTR [r11+r10*1+0x7]
  6655ef:	lea    rdi,[r13+0x4000]
  6655f6:	vpxorq xmm19,xmm19,XMMWORD PTR [rdi+rcx*1]
  6655fd:	lea    rcx,[r13+0x5000]
  665604:	vpxorq xmm20,xmm20,XMMWORD PTR [rcx+rsi*1]
  66560b:	shl    r9d,0x4
  66560f:	lea    rcx,[r13+0x6000]
  665616:	vpxorq xmm21,xmm21,XMMWORD PTR [rcx+r8*1]
  66561d:	lea    rcx,[r13+0x7000]
  665624:	vpxor  xmm14,xmm14,XMMWORD PTR [rcx+r9*1]
  66562a:	cmp    rbp,0x2
  66562e:	je     665700 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x630>
  665634:	movzx  ecx,BYTE PTR [r11+r10*1+0x8]
  66563a:	shl    ecx,0x4
  66563d:	movzx  esi,BYTE PTR [r11+r10*1+0x9]
  665643:	shl    esi,0x4
  665646:	movzx  r8d,BYTE PTR [r11+r10*1+0xa]
  66564c:	shl    r8d,0x4
  665650:	movzx  r9d,BYTE PTR [r11+r10*1+0xb]
  665656:	lea    rdi,[r13+0x8000]
  66565d:	vpxorq xmm19,xmm19,XMMWORD PTR [rdi+rcx*1]
  665664:	lea    rcx,[r13+0x9000]
  66566b:	vpxorq xmm20,xmm20,XMMWORD PTR [rcx+rsi*1]
  665672:	shl    r9d,0x4
  665676:	lea    rcx,[r13+0xa000]
  66567d:	vpxorq xmm21,xmm21,XMMWORD PTR [rcx+r8*1]
  665684:	lea    rcx,[r13+0xb000]
  66568b:	vpxor  xmm14,xmm14,XMMWORD PTR [rcx+r9*1]
  665691:	cmp    rbp,0x3
  665695:	je     665700 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x630>
  665697:	movzx  ecx,BYTE PTR [r11+r10*1+0xc]
  66569d:	shl    ecx,0x4
  6656a0:	movzx  esi,BYTE PTR [r11+r10*1+0xd]
  6656a6:	shl    esi,0x4
  6656a9:	movzx  r8d,BYTE PTR [r11+r10*1+0xe]
  6656af:	shl    r8d,0x4
  6656b3:	movzx  r9d,BYTE PTR [r11+r10*1+0xf]
  6656b9:	shl    r9d,0x4
  6656bd:	lea    rdi,[r13+0xc000]
  6656c4:	vpxorq xmm19,xmm19,XMMWORD PTR [rdi+rcx*1]
  6656cb:	lea    rcx,[r13+0xd000]
  6656d2:	vpxorq xmm20,xmm20,XMMWORD PTR [rcx+rsi*1]
  6656d9:	lea    rcx,[r13+0xe000]
  6656e0:	vpxorq xmm21,xmm21,XMMWORD PTR [rcx+r8*1]
  6656e7:	lea    rcx,[r13+0xf000]
  6656ee:	vpxor  xmm14,xmm14,XMMWORD PTR [rcx+r9*1]
  6656f4:	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  665700:	mov    r10,rax
  665703:	or     r10,0x20
  665707:	mov    rcx,rax
  66570a:	or     rcx,0x2f
  66570e:	cmp    rcx,rdx
  665711:	jae    666bb4 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ae4>
  665717:	movzx  ecx,BYTE PTR [r11+r10*1]
  66571c:	shl    ecx,0x4
  66571f:	movzx  esi,BYTE PTR [r11+r10*1+0x1]
  665725:	shl    esi,0x4
  665728:	movzx  r8d,BYTE PTR [r11+r10*1+0x2]
  66572e:	shl    r8d,0x4
  665732:	movzx  r9d,BYTE PTR [r11+r10*1+0x3]
  665738:	vmovdqa xmm12,XMMWORD PTR [r13+rcx*1+0x0]
  66573f:	lea    rcx,[r13+0x1000]
  665746:	vmovdqa xmm13,XMMWORD PTR [rcx+rsi*1]
  66574b:	shl    r9d,0x4
  66574f:	lea    rcx,[r13+0x2000]
  665756:	vmovdqa xmm11,XMMWORD PTR [rcx+r8*1]
  66575c:	lea    rcx,[r13+0x3000]
  665763:	vmovdqa64 xmm18,XMMWORD PTR [rcx+r9*1]
  66576a:	cmp    rbp,0x1
  66576e:	je     665890 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x7c0>
  665774:	movzx  ecx,BYTE PTR [r11+r10*1+0x4]
  66577a:	shl    ecx,0x4
  66577d:	movzx  esi,BYTE PTR [r11+r10*1+0x5]
  665783:	shl    esi,0x4
  665786:	movzx  r8d,BYTE PTR [r11+r10*1+0x6]
  66578c:	shl    r8d,0x4
  665790:	movzx  r9d,BYTE PTR [r11+r10*1+0x7]
  665796:	lea    rdi,[r13+0x4000]
  66579d:	vpxor  xmm12,xmm12,XMMWORD PTR [rdi+rcx*1]
  6657a2:	lea    rcx,[r13+0x5000]
  6657a9:	vpxor  xmm13,xmm13,XMMWORD PTR [rcx+rsi*1]
  6657ae:	shl    r9d,0x4
  6657b2:	lea    rcx,[r13+0x6000]
  6657b9:	vpxor  xmm11,xmm11,XMMWORD PTR [rcx+r8*1]
  6657bf:	lea    rcx,[r13+0x7000]
  6657c6:	vpxorq xmm18,xmm18,XMMWORD PTR [rcx+r9*1]
  6657cd:	cmp    rbp,0x2
  6657d1:	je     665890 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x7c0>
  6657d7:	movzx  ecx,BYTE PTR [r11+r10*1+0x8]
  6657dd:	shl    ecx,0x4
  6657e0:	movzx  esi,BYTE PTR [r11+r10*1+0x9]
  6657e6:	shl    esi,0x4
  6657e9:	movzx  r8d,BYTE PTR [r11+r10*1+0xa]
  6657ef:	shl    r8d,0x4
  6657f3:	movzx  r9d,BYTE PTR [r11+r10*1+0xb]
  6657f9:	lea    rdi,[r13+0x8000]
  665800:	vpxor  xmm12,xmm12,XMMWORD PTR [rdi+rcx*1]
  665805:	lea    rcx,[r13+0x9000]
  66580c:	vpxor  xmm13,xmm13,XMMWORD PTR [rcx+rsi*1]
  665811:	shl    r9d,0x4
  665815:	lea    rcx,[r13+0xa000]
  66581c:	vpxor  xmm11,xmm11,XMMWORD PTR [rcx+r8*1]
  665822:	lea    rcx,[r13+0xb000]
  665829:	vpxorq xmm18,xmm18,XMMWORD PTR [rcx+r9*1]
  665830:	cmp    rbp,0x3
  665834:	je     665890 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x7c0>
  665836:	movzx  ecx,BYTE PTR [r11+r10*1+0xc]
  66583c:	shl    ecx,0x4
  66583f:	movzx  esi,BYTE PTR [r11+r10*1+0xd]
  665845:	shl    esi,0x4
  665848:	movzx  r8d,BYTE PTR [r11+r10*1+0xe]
  66584e:	shl    r8d,0x4
  665852:	movzx  r9d,BYTE PTR [r11+r10*1+0xf]
  665858:	shl    r9d,0x4
  66585c:	lea    rdi,[r13+0xc000]
  665863:	vpxor  xmm12,xmm12,XMMWORD PTR [rdi+rcx*1]
  665868:	lea    rcx,[r13+0xd000]
  66586f:	vpxor  xmm13,xmm13,XMMWORD PTR [rcx+rsi*1]
  665874:	lea    rcx,[r13+0xe000]
  66587b:	vpxor  xmm11,xmm11,XMMWORD PTR [rcx+r8*1]
  665881:	lea    rcx,[r13+0xf000]
  665888:	vpxorq xmm18,xmm18,XMMWORD PTR [rcx+r9*1]
  66588f:	nop
  665890:	mov    r10,rax
  665893:	or     r10,0x30
  665897:	mov    rcx,rax
  66589a:	or     rcx,0x3f
  66589e:	cmp    rcx,rdx
  6658a1:	jae    666bb4 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ae4>
  6658a7:	movzx  ecx,BYTE PTR [r11+r10*1]
  6658ac:	shl    ecx,0x4
  6658af:	movzx  esi,BYTE PTR [r11+r10*1+0x1]
  6658b5:	shl    esi,0x4
  6658b8:	movzx  r8d,BYTE PTR [r11+r10*1+0x2]
  6658be:	shl    r8d,0x4
  6658c2:	movzx  r9d,BYTE PTR [r11+r10*1+0x3]
  6658c8:	vmovdqa xmm10,XMMWORD PTR [r13+rcx*1+0x0]
  6658cf:	lea    rcx,[r13+0x1000]
  6658d6:	vmovdqa64 xmm22,XMMWORD PTR [rcx+rsi*1]
  6658dd:	shl    r9d,0x4
  6658e1:	lea    rcx,[r13+0x2000]
  6658e8:	vmovdqa64 xmm24,XMMWORD PTR [rcx+r8*1]
  6658ef:	lea    rcx,[r13+0x3000]
  6658f6:	vmovdqa64 xmm25,XMMWORD PTR [rcx+r9*1]
  6658fd:	cmp    rbp,0x1
  665901:	je     665a30 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x960>
  665907:	movzx  ecx,BYTE PTR [r11+r10*1+0x4]
  66590d:	shl    ecx,0x4
  665910:	movzx  esi,BYTE PTR [r11+r10*1+0x5]
  665916:	shl    esi,0x4
  665919:	movzx  r8d,BYTE PTR [r11+r10*1+0x6]
  66591f:	shl    r8d,0x4
  665923:	movzx  r9d,BYTE PTR [r11+r10*1+0x7]
  665929:	lea    rdi,[r13+0x4000]
  665930:	vpxor  xmm10,xmm10,XMMWORD PTR [rdi+rcx*1]
  665935:	lea    rcx,[r13+0x5000]
  66593c:	vpxorq xmm22,xmm22,XMMWORD PTR [rcx+rsi*1]
  665943:	shl    r9d,0x4
  665947:	lea    rcx,[r13+0x6000]
  66594e:	vpxorq xmm24,xmm24,XMMWORD PTR [rcx+r8*1]
  665955:	lea    rcx,[r13+0x7000]
  66595c:	vpxorq xmm25,xmm25,XMMWORD PTR [rcx+r9*1]
  665963:	cmp    rbp,0x2
  665967:	je     665a30 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x960>
  66596d:	movzx  ecx,BYTE PTR [r11+r10*1+0x8]
  665973:	shl    ecx,0x4
  665976:	movzx  esi,BYTE PTR [r11+r10*1+0x9]
  66597c:	shl    esi,0x4
  66597f:	movzx  r8d,BYTE PTR [r11+r10*1+0xa]
  665985:	shl    r8d,0x4
  665989:	movzx  r9d,BYTE PTR [r11+r10*1+0xb]
  66598f:	lea    rdi,[r13+0x8000]
  665996:	vpxor  xmm10,xmm10,XMMWORD PTR [rdi+rcx*1]
  66599b:	lea    rcx,[r13+0x9000]
  6659a2:	vpxorq xmm22,xmm22,XMMWORD PTR [rcx+rsi*1]
  6659a9:	shl    r9d,0x4
  6659ad:	lea    rcx,[r13+0xa000]
  6659b4:	vpxorq xmm24,xmm24,XMMWORD PTR [rcx+r8*1]
  6659bb:	lea    rcx,[r13+0xb000]
  6659c2:	vpxorq xmm25,xmm25,XMMWORD PTR [rcx+r9*1]
  6659c9:	cmp    rbp,0x3
  6659cd:	je     665a30 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x960>
  6659cf:	movzx  ecx,BYTE PTR [r11+r10*1+0xc]
  6659d5:	shl    ecx,0x4
  6659d8:	movzx  esi,BYTE PTR [r11+r10*1+0xd]
  6659de:	shl    esi,0x4
  6659e1:	movzx  r8d,BYTE PTR [r11+r10*1+0xe]
  6659e7:	shl    r8d,0x4
  6659eb:	movzx  r9d,BYTE PTR [r11+r10*1+0xf]
  6659f1:	shl    r9d,0x4
  6659f5:	lea    rdi,[r13+0xc000]
  6659fc:	vpxor  xmm10,xmm10,XMMWORD PTR [rdi+rcx*1]
  665a01:	lea    rcx,[r13+0xd000]
  665a08:	vpxorq xmm22,xmm22,XMMWORD PTR [rcx+rsi*1]
  665a0f:	lea    rcx,[r13+0xe000]
  665a16:	vpxorq xmm24,xmm24,XMMWORD PTR [rcx+r8*1]
  665a1d:	lea    rcx,[r13+0xf000]
  665a24:	vpxorq xmm25,xmm25,XMMWORD PTR [rcx+r9*1]
  665a2b:	nop    DWORD PTR [rax+rax*1+0x0]
  665a30:	mov    r10,rax
  665a33:	or     r10,0x40
  665a37:	mov    rcx,rax
  665a3a:	or     rcx,0x4f
  665a3e:	cmp    rcx,rdx
  665a41:	jae    666bb4 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ae4>
  665a47:	movzx  ecx,BYTE PTR [r11+r10*1]
  665a4c:	shl    ecx,0x4
  665a4f:	movzx  esi,BYTE PTR [r11+r10*1+0x1]
  665a55:	shl    esi,0x4
  665a58:	movzx  r8d,BYTE PTR [r11+r10*1+0x2]
  665a5e:	shl    r8d,0x4
  665a62:	movzx  r9d,BYTE PTR [r11+r10*1+0x3]
  665a68:	vmovdqa64 xmm26,XMMWORD PTR [r13+rcx*1+0x0]
  665a70:	lea    rcx,[r13+0x1000]
  665a77:	vmovdqa xmm5,XMMWORD PTR [rcx+rsi*1]
  665a7c:	shl    r9d,0x4
  665a80:	lea    rcx,[r13+0x2000]
  665a87:	vmovaps xmm28,XMMWORD PTR [rcx+r8*1]
  665a8e:	lea    rcx,[r13+0x3000]
  665a95:	vmovaps xmm23,XMMWORD PTR [rcx+r9*1]
  665a9c:	cmp    rbp,0x1
  665aa0:	je     665bd0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0xb00>
  665aa6:	movzx  ecx,BYTE PTR [r11+r10*1+0x4]
  665aac:	shl    ecx,0x4
  665aaf:	movzx  esi,BYTE PTR [r11+r10*1+0x5]
  665ab5:	shl    esi,0x4
  665ab8:	movzx  r8d,BYTE PTR [r11+r10*1+0x6]
  665abe:	shl    r8d,0x4
  665ac2:	movzx  r9d,BYTE PTR [r11+r10*1+0x7]
  665ac8:	lea    rdi,[r13+0x4000]
  665acf:	vpxorq xmm26,xmm26,XMMWORD PTR [rdi+rcx*1]
  665ad6:	lea    rcx,[r13+0x5000]
  665add:	vpxor  xmm5,xmm5,XMMWORD PTR [rcx+rsi*1]
  665ae2:	shl    r9d,0x4
  665ae6:	lea    rcx,[r13+0x6000]
  665aed:	vxorps xmm28,xmm28,XMMWORD PTR [rcx+r8*1]
  665af4:	lea    rcx,[r13+0x7000]
  665afb:	vxorps xmm23,xmm23,XMMWORD PTR [rcx+r9*1]
  665b02:	cmp    rbp,0x2
  665b06:	je     665bd0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0xb00>
  665b0c:	movzx  ecx,BYTE PTR [r11+r10*1+0x8]
  665b12:	shl    ecx,0x4
  665b15:	movzx  esi,BYTE PTR [r11+r10*1+0x9]
  665b1b:	shl    esi,0x4
  665b1e:	movzx  r8d,BYTE PTR [r11+r10*1+0xa]
  665b24:	shl    r8d,0x4
  665b28:	movzx  r9d,BYTE PTR [r11+r10*1+0xb]
  665b2e:	lea    rdi,[r13+0x8000]
  665b35:	vpxorq xmm26,xmm26,XMMWORD PTR [rdi+rcx*1]
  665b3c:	lea    rcx,[r13+0x9000]
  665b43:	vpxor  xmm5,xmm5,XMMWORD PTR [rcx+rsi*1]
  665b48:	shl    r9d,0x4
  665b4c:	lea    rcx,[r13+0xa000]
  665b53:	vxorps xmm28,xmm28,XMMWORD PTR [rcx+r8*1]
  665b5a:	lea    rcx,[r13+0xb000]
  665b61:	vxorps xmm23,xmm23,XMMWORD PTR [rcx+r9*1]
  665b68:	cmp    rbp,0x3
  665b6c:	je     665bd0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0xb00>
  665b6e:	movzx  ecx,BYTE PTR [r11+r10*1+0xc]
  665b74:	shl    ecx,0x4
  665b77:	movzx  esi,BYTE PTR [r11+r10*1+0xd]
  665b7d:	shl    esi,0x4
  665b80:	movzx  r8d,BYTE PTR [r11+r10*1+0xe]
  665b86:	shl    r8d,0x4
  665b8a:	movzx  r9d,BYTE PTR [r11+r10*1+0xf]
  665b90:	shl    r9d,0x4
  665b94:	lea    rdi,[r13+0xc000]
  665b9b:	vpxorq xmm26,xmm26,XMMWORD PTR [rdi+rcx*1]
  665ba2:	lea    rcx,[r13+0xd000]
  665ba9:	vpxor  xmm5,xmm5,XMMWORD PTR [rcx+rsi*1]
  665bae:	lea    rcx,[r13+0xe000]
  665bb5:	vxorps xmm28,xmm28,XMMWORD PTR [rcx+r8*1]
  665bbc:	lea    rcx,[r13+0xf000]
  665bc3:	vxorps xmm23,xmm23,XMMWORD PTR [rcx+r9*1]
  665bca:	nop    WORD PTR [rax+rax*1+0x0]
  665bd0:	mov    r10,rax
  665bd3:	or     r10,0x50
  665bd7:	mov    rcx,rax
  665bda:	or     rcx,0x5f
  665bde:	cmp    rcx,rdx
  665be1:	jae    666bb4 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ae4>
  665be7:	movzx  ecx,BYTE PTR [r11+r10*1]
  665bec:	shl    ecx,0x4
  665bef:	movzx  esi,BYTE PTR [r11+r10*1+0x1]
  665bf5:	shl    esi,0x4
  665bf8:	movzx  r8d,BYTE PTR [r11+r10*1+0x2]
  665bfe:	shl    r8d,0x4
  665c02:	movzx  r9d,BYTE PTR [r11+r10*1+0x3]
  665c08:	vmovaps xmm29,XMMWORD PTR [r13+rcx*1+0x0]
  665c10:	lea    rcx,[r13+0x1000]
  665c17:	vmovdqa xmm6,XMMWORD PTR [rcx+rsi*1]
  665c1c:	shl    r9d,0x4
  665c20:	lea    rcx,[r13+0x2000]
  665c27:	vmovdqa xmm8,XMMWORD PTR [rcx+r8*1]
  665c2d:	lea    rcx,[r13+0x3000]
  665c34:	vmovdqa64 xmm30,XMMWORD PTR [rcx+r9*1]
  665c3b:	cmp    rbp,0x1
  665c3f:	je     665d70 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0xca0>
  665c45:	movzx  ecx,BYTE PTR [r11+r10*1+0x4]
  665c4b:	shl    ecx,0x4
  665c4e:	movzx  esi,BYTE PTR [r11+r10*1+0x5]
  665c54:	shl    esi,0x4
  665c57:	movzx  r8d,BYTE PTR [r11+r10*1+0x6]
  665c5d:	shl    r8d,0x4
  665c61:	movzx  r9d,BYTE PTR [r11+r10*1+0x7]
  665c67:	lea    rdi,[r13+0x4000]
  665c6e:	vxorps xmm29,xmm29,XMMWORD PTR [rdi+rcx*1]
  665c75:	lea    rcx,[r13+0x5000]
  665c7c:	vpxor  xmm6,xmm6,XMMWORD PTR [rcx+rsi*1]
  665c81:	shl    r9d,0x4
  665c85:	lea    rcx,[r13+0x6000]
  665c8c:	vpxor  xmm8,xmm8,XMMWORD PTR [rcx+r8*1]
  665c92:	lea    rcx,[r13+0x7000]
  665c99:	vpxorq xmm30,xmm30,XMMWORD PTR [rcx+r9*1]
  665ca0:	cmp    rbp,0x2
  665ca4:	je     665d70 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0xca0>
  665caa:	movzx  ecx,BYTE PTR [r11+r10*1+0x8]
  665cb0:	shl    ecx,0x4
  665cb3:	movzx  esi,BYTE PTR [r11+r10*1+0x9]
  665cb9:	shl    esi,0x4
  665cbc:	movzx  r8d,BYTE PTR [r11+r10*1+0xa]
  665cc2:	shl    r8d,0x4
  665cc6:	movzx  r9d,BYTE PTR [r11+r10*1+0xb]
  665ccc:	lea    rdi,[r13+0x8000]
  665cd3:	vxorps xmm29,xmm29,XMMWORD PTR [rdi+rcx*1]
  665cda:	lea    rcx,[r13+0x9000]
  665ce1:	vpxor  xmm6,xmm6,XMMWORD PTR [rcx+rsi*1]
  665ce6:	shl    r9d,0x4
  665cea:	lea    rcx,[r13+0xa000]
  665cf1:	vpxor  xmm8,xmm8,XMMWORD PTR [rcx+r8*1]
  665cf7:	lea    rcx,[r13+0xb000]
  665cfe:	vpxorq xmm30,xmm30,XMMWORD PTR [rcx+r9*1]
  665d05:	cmp    rbp,0x3
  665d09:	je     665d70 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0xca0>
  665d0b:	movzx  ecx,BYTE PTR [r11+r10*1+0xc]
  665d11:	shl    ecx,0x4
  665d14:	movzx  esi,BYTE PTR [r11+r10*1+0xd]
  665d1a:	shl    esi,0x4
  665d1d:	movzx  r8d,BYTE PTR [r11+r10*1+0xe]
  665d23:	shl    r8d,0x4
  665d27:	movzx  r9d,BYTE PTR [r11+r10*1+0xf]
  665d2d:	shl    r9d,0x4
  665d31:	lea    rdi,[r13+0xc000]
  665d38:	vxorps xmm29,xmm29,XMMWORD PTR [rdi+rcx*1]
  665d3f:	lea    rcx,[r13+0xd000]
  665d46:	vpxor  xmm6,xmm6,XMMWORD PTR [rcx+rsi*1]
  665d4b:	lea    rcx,[r13+0xe000]
  665d52:	vpxor  xmm8,xmm8,XMMWORD PTR [rcx+r8*1]
  665d58:	lea    rcx,[r13+0xf000]
  665d5f:	vpxorq xmm30,xmm30,XMMWORD PTR [rcx+r9*1]
  665d66:	cs nop WORD PTR [rax+rax*1+0x0]
  665d70:	mov    r10,rax
  665d73:	or     r10,0x60
  665d77:	mov    rcx,rax
  665d7a:	or     rcx,0x6f
  665d7e:	cmp    rcx,rdx
  665d81:	jae    666bb4 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ae4>
  665d87:	vmovdqa XMMWORD PTR [rsp+0x50],xmm8
  665d8d:	vmovdqa XMMWORD PTR [rsp+0x80],xmm6
  665d96:	movzx  ecx,BYTE PTR [r11+r10*1]
  665d9b:	shl    ecx,0x4
  665d9e:	movzx  esi,BYTE PTR [r11+r10*1+0x1]
  665da4:	shl    esi,0x4
  665da7:	movzx  r8d,BYTE PTR [r11+r10*1+0x2]
  665dad:	shl    r8d,0x4
  665db1:	movzx  r9d,BYTE PTR [r11+r10*1+0x3]
  665db7:	vmovdqa64 xmm31,XMMWORD PTR [r13+rcx*1+0x0]
  665dbf:	lea    rcx,[r13+0x1000]
  665dc6:	vmovdqa xmm6,XMMWORD PTR [rcx+rsi*1]
  665dcb:	shl    r9d,0x4
  665dcf:	lea    rcx,[r13+0x2000]
  665dd6:	vmovdqa xmm8,XMMWORD PTR [rcx+r8*1]
  665ddc:	lea    rcx,[r13+0x3000]
  665de3:	vmovdqa xmm9,XMMWORD PTR [rcx+r9*1]
  665de9:	cmp    rbp,0x1
  665ded:	je     665f20 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0xe50>
  665df3:	movzx  ecx,BYTE PTR [r11+r10*1+0x4]
  665df9:	shl    ecx,0x4
  665dfc:	movzx  esi,BYTE PTR [r11+r10*1+0x5]
  665e02:	shl    esi,0x4
  665e05:	movzx  r8d,BYTE PTR [r11+r10*1+0x6]
  665e0b:	shl    r8d,0x4
  665e0f:	movzx  r9d,BYTE PTR [r11+r10*1+0x7]
  665e15:	lea    rdi,[r13+0x4000]
  665e1c:	vpxorq xmm31,xmm31,XMMWORD PTR [rdi+rcx*1]
  665e23:	lea    rcx,[r13+0x5000]
  665e2a:	vpxor  xmm6,xmm6,XMMWORD PTR [rcx+rsi*1]
  665e2f:	shl    r9d,0x4
  665e33:	lea    rcx,[r13+0x6000]
  665e3a:	vpxor  xmm8,xmm8,XMMWORD PTR [rcx+r8*1]
  665e40:	lea    rcx,[r13+0x7000]
  665e47:	vpxor  xmm9,xmm9,XMMWORD PTR [rcx+r9*1]
  665e4d:	cmp    rbp,0x2
  665e51:	je     665f20 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0xe50>
  665e57:	movzx  ecx,BYTE PTR [r11+r10*1+0x8]
  665e5d:	shl    ecx,0x4
  665e60:	movzx  esi,BYTE PTR [r11+r10*1+0x9]
  665e66:	shl    esi,0x4
  665e69:	movzx  r8d,BYTE PTR [r11+r10*1+0xa]
  665e6f:	shl    r8d,0x4
  665e73:	movzx  r9d,BYTE PTR [r11+r10*1+0xb]
  665e79:	lea    rdi,[r13+0x8000]
  665e80:	vpxorq xmm31,xmm31,XMMWORD PTR [rdi+rcx*1]
  665e87:	lea    rcx,[r13+0x9000]
  665e8e:	vpxor  xmm6,xmm6,XMMWORD PTR [rcx+rsi*1]
  665e93:	shl    r9d,0x4
  665e97:	lea    rcx,[r13+0xa000]
  665e9e:	vpxor  xmm8,xmm8,XMMWORD PTR [rcx+r8*1]
  665ea4:	lea    rcx,[r13+0xb000]
  665eab:	vpxor  xmm9,xmm9,XMMWORD PTR [rcx+r9*1]
  665eb1:	cmp    rbp,0x3
  665eb5:	je     665f20 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0xe50>
  665eb7:	movzx  ecx,BYTE PTR [r11+r10*1+0xc]
  665ebd:	shl    ecx,0x4
  665ec0:	movzx  esi,BYTE PTR [r11+r10*1+0xd]
  665ec6:	shl    esi,0x4
  665ec9:	movzx  r8d,BYTE PTR [r11+r10*1+0xe]
  665ecf:	shl    r8d,0x4
  665ed3:	movzx  r9d,BYTE PTR [r11+r10*1+0xf]
  665ed9:	shl    r9d,0x4
  665edd:	lea    rdi,[r13+0xc000]
  665ee4:	vpxorq xmm31,xmm31,XMMWORD PTR [rdi+rcx*1]
  665eeb:	lea    rcx,[r13+0xd000]
  665ef2:	vpxor  xmm6,xmm6,XMMWORD PTR [rcx+rsi*1]
  665ef7:	lea    rcx,[r13+0xe000]
  665efe:	vpxor  xmm8,xmm8,XMMWORD PTR [rcx+r8*1]
  665f04:	lea    rcx,[r13+0xf000]
  665f0b:	vpxor  xmm9,xmm9,XMMWORD PTR [rcx+r9*1]
  665f11:	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  665f20:	mov    r10,rax
  665f23:	or     r10,0x70
  665f27:	or     rax,0x7f
  665f2b:	cmp    rax,rdx
  665f2e:	jae    666bb4 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ae4>
  665f34:	vmovaps XMMWORD PTR [rsp+0x60],xmm29
  665f3c:	vmovaps XMMWORD PTR [rsp+0x70],xmm28
  665f44:	vmovups ZMMWORD PTR [rsp+0xd0],zmm23
  665f4f:	vmovdqa64 xmm23,xmm26
  665f55:	vmovdqa64 xmm29,xmm25
  665f5b:	vmovdqa64 zmm28,zmm2
  665f61:	vmovdqa64 zmm2,zmm1
  665f67:	vmovdqa64 zmm1,zmm27
  665f6d:	vmovdqa64 xmm27,xmm22
  665f73:	vmovdqa64 xmm22,xmm10
  665f79:	vmovdqa64 xmm26,xmm11
  665f7f:	vmovdqa64 xmm25,xmm13
  665f85:	vmovdqu64 ZMMWORD PTR [rsp+0x110],zmm0
  665f90:	vmovdqu64 ZMMWORD PTR [rsp+0x150],zmm4
  665f9b:	vmovdqa xmm0,xmm12
  665f9f:	movzx  eax,BYTE PTR [r11+r10*1]
  665fa4:	shl    eax,0x4
  665fa7:	movzx  ecx,BYTE PTR [r11+r10*1+0x1]
  665fad:	shl    ecx,0x4
  665fb0:	movzx  esi,BYTE PTR [r11+r10*1+0x2]
  665fb6:	shl    esi,0x4
  665fb9:	movzx  r8d,BYTE PTR [r11+r10*1+0x3]
  665fbf:	vmovdqa xmm10,XMMWORD PTR [r13+rax*1+0x0]
  665fc6:	lea    rax,[r13+0x1000]
  665fcd:	vmovdqa xmm11,XMMWORD PTR [rax+rcx*1]
  665fd2:	shl    r8d,0x4
  665fd6:	lea    rax,[r13+0x2000]
  665fdd:	vmovdqa xmm12,XMMWORD PTR [rax+rsi*1]
  665fe2:	lea    rax,[r13+0x3000]
  665fe9:	vmovdqa xmm13,XMMWORD PTR [rax+r8*1]
  665fef:	cmp    rbp,0x1
  665ff3:	je     666110 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1040>
  665ff9:	movzx  eax,BYTE PTR [r11+r10*1+0x4]
  665fff:	shl    eax,0x4
  666002:	movzx  ecx,BYTE PTR [r11+r10*1+0x5]
  666008:	shl    ecx,0x4
  66600b:	movzx  esi,BYTE PTR [r11+r10*1+0x6]
  666011:	shl    esi,0x4
  666014:	movzx  r8d,BYTE PTR [r11+r10*1+0x7]
  66601a:	lea    rdi,[r13+0x4000]
  666021:	vpxor  xmm10,xmm10,XMMWORD PTR [rdi+rax*1]
  666026:	lea    rax,[r13+0x5000]
  66602d:	vpxor  xmm11,xmm11,XMMWORD PTR [rax+rcx*1]
  666032:	shl    r8d,0x4
  666036:	lea    rax,[r13+0x6000]
  66603d:	vpxor  xmm12,xmm12,XMMWORD PTR [rax+rsi*1]
  666042:	lea    rax,[r13+0x7000]
  666049:	vpxor  xmm13,xmm13,XMMWORD PTR [rax+r8*1]
  66604f:	cmp    rbp,0x2
  666053:	je     666110 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1040>
  666059:	movzx  eax,BYTE PTR [r11+r10*1+0x8]
  66605f:	shl    eax,0x4
  666062:	movzx  ecx,BYTE PTR [r11+r10*1+0x9]
  666068:	shl    ecx,0x4
  66606b:	movzx  esi,BYTE PTR [r11+r10*1+0xa]
  666071:	shl    esi,0x4
  666074:	movzx  r8d,BYTE PTR [r11+r10*1+0xb]
  66607a:	lea    rdi,[r13+0x8000]
  666081:	vpxor  xmm10,xmm10,XMMWORD PTR [rdi+rax*1]
  666086:	lea    rax,[r13+0x9000]
  66608d:	vpxor  xmm11,xmm11,XMMWORD PTR [rax+rcx*1]
  666092:	shl    r8d,0x4
  666096:	lea    rax,[r13+0xa000]
  66609d:	vpxor  xmm12,xmm12,XMMWORD PTR [rax+rsi*1]
  6660a2:	lea    rax,[r13+0xb000]
  6660a9:	vpxor  xmm13,xmm13,XMMWORD PTR [rax+r8*1]
  6660af:	cmp    rbp,0x3
  6660b3:	je     666110 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1040>
  6660b5:	movzx  eax,BYTE PTR [r11+r10*1+0xc]
  6660bb:	shl    eax,0x4
  6660be:	movzx  ecx,BYTE PTR [r11+r10*1+0xd]
  6660c4:	shl    ecx,0x4
  6660c7:	movzx  esi,BYTE PTR [r11+r10*1+0xe]
  6660cd:	shl    esi,0x4
  6660d0:	movzx  r8d,BYTE PTR [r11+r10*1+0xf]
  6660d6:	shl    r8d,0x4
  6660da:	lea    rdi,[r13+0xc000]
  6660e1:	vpxor  xmm10,xmm10,XMMWORD PTR [rdi+rax*1]
  6660e6:	lea    rax,[r13+0xd000]
  6660ed:	vpxor  xmm11,xmm11,XMMWORD PTR [rax+rcx*1]
  6660f2:	lea    rax,[r13+0xe000]
  6660f9:	vpxor  xmm12,xmm12,XMMWORD PTR [rax+rsi*1]
  6660fe:	lea    rax,[r13+0xf000]
  666105:	vpxor  xmm13,xmm13,XMMWORD PTR [rax+r8*1]
  66610b:	nop    DWORD PTR [rax+rax*1+0x0]
  666110:	vpxor  xmm10,xmm11,xmm10
  666115:	vpternlogq xmm10,xmm13,xmm12,0x96
  66611c:	vpxorq xmm11,xmm16,xmm15
  666122:	vpternlogq xmm7,xmm11,xmm17,0x96
  666129:	vpxorq xmm11,xmm20,xmm19
  66612f:	vpternlogq xmm14,xmm11,xmm21,0x96
  666136:	vinserti32x4 zmm7,zmm7,xmm14,0x1
  66613d:	vpxorq xmm11,xmm25,xmm0
  666143:	vpternlogq xmm18,xmm11,xmm26,0x96
  66614a:	vinserti32x4 zmm7,zmm7,xmm18,0x2
  666151:	vpxorq xmm11,xmm27,xmm22
  666157:	vpternlogq xmm29,xmm11,xmm24,0x96
  66615e:	vinserti32x4 zmm7,zmm7,xmm29,0x3
  666165:	vpxorq xmm11,xmm5,xmm23
  66616b:	vmovdqu64 zmm5,ZMMWORD PTR [rsp+0xd0]
  666176:	vpternlogq xmm5,xmm11,XMMWORD PTR [rsp+0x70],0x96
  66617f:	vmovdqa xmm0,XMMWORD PTR [rsp+0x80]
  666188:	vpxor  xmm11,xmm0,XMMWORD PTR [rsp+0x60]
  66618e:	vpternlogq xmm30,xmm11,XMMWORD PTR [rsp+0x50],0x96
  666197:	vinserti32x4 zmm11,zmm5,xmm30,0x1
  66619e:	vpxorq xmm6,xmm6,xmm31
  6661a4:	vpternlogq xmm9,xmm6,xmm8,0x96
  6661ab:	vinserti32x4 zmm6,zmm11,xmm9,0x2
  6661b2:	vinserti32x4 zmm6,zmm6,xmm10,0x3
  6661b9:	mov    rax,QWORD PTR [rsp+0x10]
  6661be:	xor    esi,esi
  6661c0:	vmovdqa64 zmm27,zmm1
  6661c6:	vmovdqa64 zmm1,zmm2
  6661cc:	vmovdqa64 zmm2,zmm28
  6661d2:	vmovdqu64 zmm4,ZMMWORD PTR [rsp+0x150]
  6661dd:	vmovdqu64 zmm0,ZMMWORD PTR [rsp+0x110]
  6661e8:	jmp    66628d <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x11bd>
  6661ed:	nop    DWORD PTR [rax]
  6661f0:	xor    r9d,r9d
  6661f3:	vpxor  xmm9,xmm9,xmm9
  6661f8:	vpxor  xmm10,xmm10,xmm10
  6661fd:	vpxor  xmm11,xmm11,xmm11
  666202:	mov    rcx,QWORD PTR [rsp+0x8]
  666207:	add    rcx,rbx
  66620a:	add    r10,rcx
  66620d:	mov    rcx,r9
  666210:	shl    rcx,0xe
  666214:	add    rcx,r13
  666217:	movzx  r8d,BYTE PTR [r10+r9*4]
  66621c:	shl    r8d,0x4
  666220:	movzx  r12d,BYTE PTR [r10+r9*4+0x1]
  666226:	shl    r12d,0x4
  66622a:	movzx  edi,BYTE PTR [r10+r9*4+0x2]
  666230:	shl    edi,0x4
  666233:	movzx  r9d,BYTE PTR [r10+r9*4+0x3]
  666239:	shl    r9d,0x4
  66623d:	vpxor  xmm9,xmm9,XMMWORD PTR [r8+rcx*1]
  666243:	vpxor  xmm8,xmm8,XMMWORD PTR [r12+rcx*1+0x1000]
  66624d:	vpxor  xmm10,xmm10,XMMWORD PTR [rdi+rcx*1+0x2000]
  666256:	vpxor  xmm11,xmm11,XMMWORD PTR [r9+rcx*1+0x3000]
  666260:	vpxor  xmm8,xmm8,xmm9
  666265:	vpternlogq xmm8,xmm11,xmm10,0x96
  66626c:	mov    rcx,rsi
  66626f:	shl    rcx,0x4
  666273:	vmovdqa XMMWORD PTR [rsp+rcx*1+0x190],xmm8
  66627c:	inc    rsi
  66627f:	add    rax,0x10
  666283:	cmp    rsi,0x8
  666287:	je     665210 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x140>
  66628d:	lea    r10,[rsi+r15*1]
  666291:	shl    r10,0x4
  666295:	lea    rcx,[r10+0xf]
  666299:	cmp    rcx,rdx
  66629c:	jae    666b54 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1a84>
  6662a2:	vpxor  xmm8,xmm8,xmm8
  6662a7:	cmp    rbp,0x1
  6662ab:	je     6661f0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1120>
  6662b1:	lea    r8,[r13+0x7000]
  6662b8:	xor    r9d,r9d
  6662bb:	vpxor  xmm9,xmm9,xmm9
  6662c0:	vpxor  xmm10,xmm10,xmm10
  6662c5:	vpxor  xmm11,xmm11,xmm11
  6662ca:	nop    WORD PTR [rax+rax*1+0x0]
  6662d0:	movzx  ecx,BYTE PTR [rax+r9*4-0x7]
  6662d6:	shl    ecx,0x4
  6662d9:	movzx  edi,BYTE PTR [rax+r9*4-0x6]
  6662df:	shl    edi,0x4
  6662e2:	movzx  r12d,BYTE PTR [rax+r9*4-0x5]
  6662e8:	shl    r12d,0x4
  6662ec:	movzx  ebx,BYTE PTR [rax+r9*4-0x4]
  6662f2:	vmovdqa xmm12,XMMWORD PTR [r8+rcx*1-0x7000]
  6662fc:	vmovdqa xmm13,XMMWORD PTR [r8+rdi*1-0x6000]
  666306:	shl    ebx,0x4
  666309:	vmovdqa xmm14,XMMWORD PTR [r8+r12*1-0x5000]
  666313:	vmovdqa xmm15,XMMWORD PTR [r8+rbx*1-0x4000]
  66631d:	movzx  ecx,BYTE PTR [rax+r9*4-0x3]
  666323:	movzx  edi,BYTE PTR [rax+r9*4-0x2]
  666329:	movzx  ebx,BYTE PTR [rax+r9*4-0x1]
  66632f:	movzx  r12d,BYTE PTR [rax+r9*4]
  666334:	add    r9,0x2
  666338:	shl    ecx,0x4
  66633b:	shl    edi,0x4
  66633e:	shl    ebx,0x4
  666341:	shl    r12d,0x4
  666345:	vpternlogq xmm9,xmm12,XMMWORD PTR [r8+rcx*1-0x3000],0x96
  666351:	vpternlogq xmm8,xmm13,XMMWORD PTR [r8+rdi*1-0x2000],0x96
  66635d:	vpternlogq xmm10,xmm14,XMMWORD PTR [r8+rbx*1-0x1000],0x96
  666369:	vpternlogq xmm11,xmm15,XMMWORD PTR [r8+r12*1],0x96
  666371:	add    r8,0x8000
  666378:	cmp    r14,r9
  66637b:	jne    6662d0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1200>
  666381:	test   BYTE PTR [rsp+0x28],0x1
  666386:	mov    rbx,QWORD PTR [rsp]
  66638a:	jne    666202 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1132>
  666390:	jmp    666260 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1190>
  666395:	data16 cs nop WORD PTR [rax+rax*1+0x0]
  6663a0:	cmp    rax,rdx
  6663a3:	jae    666bb4 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ae4>
  6663a9:	mov    rax,r10
  6663ac:	or     rax,0x1f
  6663b0:	cmp    rax,rdx
  6663b3:	jae    666b82 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ab2>
  6663b9:	mov    rax,r10
  6663bc:	or     rax,0x2f
  6663c0:	cmp    rax,rdx
  6663c3:	jae    666b89 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ab9>
  6663c9:	mov    rax,r10
  6663cc:	or     rax,0x3f
  6663d0:	cmp    rax,rdx
  6663d3:	jae    666b90 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ac0>
  6663d9:	mov    rax,r10
  6663dc:	or     rax,0x4f
  6663e0:	cmp    rax,rdx
  6663e3:	jae    666b97 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ac7>
  6663e9:	mov    rax,r10
  6663ec:	or     rax,0x5f
  6663f0:	cmp    rax,rdx
  6663f3:	jae    666b9e <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ace>
  6663f9:	mov    rax,r10
  6663fc:	or     rax,0x6f
  666400:	cmp    rax,rdx
  666403:	jae    666ba5 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ad5>
  666409:	mov    rax,r10
  66640c:	or     rax,0x7f
  666410:	cmp    rax,rdx
  666413:	jae    666bac <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1adc>
  666419:	vpxor  xmm8,xmm8,xmm8
  66641e:	vpxor  xmm9,xmm9,xmm9
  666423:	vpxor  xmm6,xmm6,xmm6
  666427:	vpxor  xmm7,xmm7,xmm7
  66642b:	jmp    66522b <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x15b>
  666430:	vpxord xmm27,xmm27,xmm27
  666436:	vpxor  xmm1,xmm1,xmm1
  66643a:	vpxor  xmm2,xmm2,xmm2
  66643e:	vpxor  xmm3,xmm3,xmm3
  666442:	vpxor  xmm4,xmm4,xmm4
  666446:	vpxor  xmm0,xmm0,xmm0
  66644a:	jmp    66645b <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x138b>
  66644c:	mov    r8,QWORD PTR [rsp+0x30]
  666451:	mov    rdi,QWORD PTR [rsp+0x8]
  666456:	mov    r9,QWORD PTR [rsp+0x18]
  66645b:	movabs rcx,0x7fffffffffffff8
  666465:	and    rcx,r8
  666468:	vextracti128 xmm7,ymm0,0x1
  66646e:	vextracti32x4 xmm6,zmm0,0x2
  666475:	vextracti32x4 xmm8,zmm0,0x3
  66647c:	vpxor  xmm6,xmm8,xmm6
  666480:	vpternlogd xmm6,xmm7,xmm0,0x96
  666487:	vextracti128 xmm5,ymm4,0x1
  66648d:	vextracti32x4 xmm7,zmm4,0x2
  666494:	vextracti32x4 xmm8,zmm4,0x3
  66649b:	vpxor  xmm7,xmm8,xmm7
  66649f:	vpternlogd xmm7,xmm5,xmm4,0x96
  6664a6:	vextracti128 xmm4,ymm3,0x1
  6664ac:	vextracti32x4 xmm5,zmm3,0x2
  6664b3:	vextracti32x4 xmm8,zmm3,0x3
  6664ba:	vpternlogd xmm8,xmm3,xmm5,0x96
  6664c1:	vpalignr xmm3,xmm7,xmm6,0x8
  6664c7:	vpternlogd xmm3,xmm8,xmm4,0x96
  6664ce:	vextracti128 xmm4,ymm2,0x1
  6664d4:	vextracti32x4 xmm5,zmm2,0x2
  6664db:	vextracti32x4 xmm8,zmm2,0x3
  6664e2:	vpxor  xmm5,xmm8,xmm5
  6664e6:	vpternlogd xmm5,xmm4,xmm2,0x96
  6664ed:	vextracti128 xmm2,ymm1,0x1
  6664f3:	vextracti32x4 xmm4,zmm1,0x2
  6664fa:	vextracti32x4 xmm8,zmm1,0x3
  666501:	vpxor  xmm4,xmm8,xmm4
  666505:	vpternlogd xmm4,xmm2,xmm1,0x96
  66650c:	vextracti32x4 xmm1,ymm27,0x1
  666513:	vextracti32x4 xmm2,zmm27,0x2
  66651a:	vextracti32x4 xmm8,zmm27,0x3
  666521:	vpternlogd xmm8,xmm27,xmm2,0x96
  666528:	vpalignr xmm0,xmm4,xmm5,0x8
  66652e:	vpternlogq xmm0,xmm8,xmm1,0x96
  666535:	vpalignr xmm1,xmm5,xmm7,0x8
  66653b:	vinserti128 ymm2,ymm3,xmm1,0x1
  666541:	vpblendd xmm1,xmm6,xmm4,0xc
  666547:	cmp    rcx,r8
  66654a:	je     666b22 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1a52>
  666550:	mov    rsi,QWORD PTR [r9+0x8]
  666554:	mov    eax,0x33
  666559:	bzhi   r15,QWORD PTR [r9+0x10],rax
  66655f:	shr    r15,0x2
  666563:	cmp    r15,0x4
  666567:	mov    r10d,0x4
  66656d:	cmovb  r10,r15
  666571:	lea    rax,[rsi+0x1000]
  666578:	mov    QWORD PTR [rsp+0x28],rax
  66657d:	lea    rax,[rsi+0x2000]
  666584:	mov    QWORD PTR [rsp+0x10],rax
  666589:	lea    rax,[rsi+0x3000]
  666590:	mov    QWORD PTR [rsp+0x80],rax
  666598:	lea    rax,[rsi+0x4000]
  66659f:	mov    QWORD PTR [rsp+0x110],rax
  6665a7:	lea    rax,[rsi+0x5000]
  6665ae:	mov    QWORD PTR [rsp+0xd0],rax
  6665b6:	lea    rax,[rsi+0x6000]
  6665bd:	mov    QWORD PTR [rsp+0x70],rax
  6665c2:	lea    rax,[rsi+0x7000]
  6665c9:	mov    QWORD PTR [rsp+0x50],rax
  6665ce:	lea    rax,[rsi+0x8000]
  6665d5:	mov    QWORD PTR [rsp+0x60],rax
  6665da:	lea    rax,[rsi+0x9000]
  6665e1:	mov    QWORD PTR [rsp+0x20],rax
  6665e6:	mov    rax,rdx
  6665e9:	and    rax,0xffffffffffffff80
  6665ed:	add    rax,rbx
  6665f0:	lea    r9,[rsi+0xa000]
  6665f7:	mov    QWORD PTR [rsp+0x18],r9
  6665fc:	lea    r13,[rdi+rax*1+0x7]
  666601:	mov    eax,0x87
  666606:	vmovq  xmm3,rax
  66660b:	mov    QWORD PTR [rsp+0x150],r10
  666613:	and    r10d,0x6
  666617:	lea    rax,[rsi+0xb000]
  66661e:	mov    QWORD PTR [rsp+0xb8],rax
  666626:	lea    rax,[rsi+0xc000]
  66662d:	mov    QWORD PTR [rsp+0xa8],rax
  666635:	lea    rax,[rsi+0xd000]
  66663c:	mov    QWORD PTR [rsp+0xa0],rax
  666644:	lea    rax,[rsi+0xe000]
  66664b:	mov    QWORD PTR [rsp+0x98],rax
  666653:	mov    QWORD PTR [rsp+0x8],rsi
  666658:	lea    rax,[rsi+0xf000]
  66665f:	mov    QWORD PTR [rsp+0x90],rax
  666667:	jmp    66671d <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x164d>
  66666c:	nop    DWORD PTR [rax+0x0]
  666670:	vmovq  xmm6,xmm4
  666674:	vmovq  xmm7,r12
  666679:	vpclmullqlqdq xmm8,xmm6,xmm7
  66667f:	mov    rax,QWORD PTR [rsp]
  666683:	vmovq  xmm9,rax
  666688:	vpclmullqlqdq xmm6,xmm6,xmm9
  66668e:	vpsrldq xmm10,xmm4,0x8
  666693:	vpclmullqlqdq xmm7,xmm10,xmm7
  666699:	vpclmullqlqdq xmm9,xmm10,xmm9
  66669f:	vpxor  xmm5,xmm5,xmm4
  6666a3:	xor    rsi,r12
  6666a6:	xor    r9,rax
  6666a9:	vmovq  xmm10,xmm5
  6666ad:	vmovq  xmm11,rsi
  6666b2:	vpclmullqlqdq xmm12,xmm10,xmm11
  6666b8:	vmovq  xmm13,r9
  6666bd:	vpclmullqlqdq xmm10,xmm10,xmm13
  6666c3:	vpsrldq xmm5,xmm5,0x8
  6666c8:	vpclmullqlqdq xmm11,xmm5,xmm11
  6666ce:	vpxor  xmm10,xmm11,xmm10
  6666d3:	vpclmullqlqdq xmm5,xmm5,xmm13
  6666d9:	vpalignr xmm11,xmm12,xmm9,0x8
  6666df:	vpalignr xmm9,xmm9,xmm8,0x8
  6666e5:	vpternlogq xmm9,xmm7,xmm6,0x96
  6666ec:	vinserti128 ymm6,ymm9,xmm11,0x1
  6666f2:	vpxor  ymm2,ymm6,ymm2
  6666f6:	vpalignr xmm6,xmm5,xmm12,0x8
  6666fc:	vpternlogq xmm0,xmm10,xmm6,0x96
  666703:	vpblendd xmm5,xmm8,xmm5,0xc
  666709:	vpxor  xmm1,xmm5,xmm1
  66670d:	inc    rcx
  666710:	add    r13,0x10
  666714:	cmp    rcx,r8
  666717:	jae    666b22 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1a52>
  66671d:	mov    rax,rcx
  666720:	shl    rax,0x4
  666724:	lea    rsi,[rax+0xf]
  666728:	cmp    rsi,rdx
  66672b:	jae    666b6b <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1a9b>
  666731:	test   r15,r15
  666734:	je     6668e0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1810>
  66673a:	movzx  esi,BYTE PTR [r11+rax*1]
  66673f:	shl    esi,0x4
  666742:	movzx  r9d,BYTE PTR [r11+rax*1+0x1]
  666748:	shl    r9d,0x4
  66674c:	movzx  ebx,BYTE PTR [r11+rax*1+0x2]
  666752:	shl    ebx,0x4
  666755:	movzx  ebp,BYTE PTR [r11+rax*1+0x3]
  66675b:	mov    rdi,QWORD PTR [rsp+0x8]
  666760:	vmovdqa xmm6,XMMWORD PTR [rdi+rsi*1]
  666765:	mov    rsi,QWORD PTR [rsp+0x28]
  66676a:	vmovdqa xmm7,XMMWORD PTR [rsi+r9*1]
  666770:	shl    ebp,0x4
  666773:	mov    rsi,QWORD PTR [rsp+0x10]
  666778:	vmovdqa xmm8,XMMWORD PTR [rsi+rbx*1]
  66677d:	mov    rsi,QWORD PTR [rsp+0x80]
  666785:	vmovdqa xmm5,XMMWORD PTR [rsi+rbp*1]
  66678a:	cmp    r15,0x1
  66678e:	je     6668b0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x17e0>
  666794:	movzx  esi,BYTE PTR [r11+rax*1+0x4]
  66679a:	shl    esi,0x4
  66679d:	movzx  r9d,BYTE PTR [r11+rax*1+0x5]
  6667a3:	shl    r9d,0x4
  6667a7:	movzx  ebx,BYTE PTR [r11+rax*1+0x6]
  6667ad:	shl    ebx,0x4
  6667b0:	movzx  ebp,BYTE PTR [r11+rax*1+0x7]
  6667b6:	mov    rdi,QWORD PTR [rsp+0x110]
  6667be:	vpxor  xmm6,xmm6,XMMWORD PTR [rdi+rsi*1]
  6667c3:	mov    rsi,QWORD PTR [rsp+0xd0]
  6667cb:	vpxor  xmm7,xmm7,XMMWORD PTR [rsi+r9*1]
  6667d1:	shl    ebp,0x4
  6667d4:	mov    rsi,QWORD PTR [rsp+0x70]
  6667d9:	vpxor  xmm8,xmm8,XMMWORD PTR [rsi+rbx*1]
  6667de:	mov    rsi,QWORD PTR [rsp+0x50]
  6667e3:	vpxor  xmm5,xmm5,XMMWORD PTR [rsi+rbp*1]
  6667e8:	cmp    r15,0x2
  6667ec:	je     6668b0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x17e0>
  6667f2:	movzx  esi,BYTE PTR [r11+rax*1+0x8]
  6667f8:	shl    esi,0x4
  6667fb:	movzx  r9d,BYTE PTR [r11+rax*1+0x9]
  666801:	shl    r9d,0x4
  666805:	movzx  ebx,BYTE PTR [r11+rax*1+0xa]
  66680b:	shl    ebx,0x4
  66680e:	movzx  ebp,BYTE PTR [r11+rax*1+0xb]
  666814:	mov    rdi,QWORD PTR [rsp+0x60]
  666819:	vpxor  xmm6,xmm6,XMMWORD PTR [rdi+rsi*1]
  66681e:	mov    rsi,QWORD PTR [rsp+0x20]
  666823:	vpxor  xmm7,xmm7,XMMWORD PTR [rsi+r9*1]
  666829:	shl    ebp,0x4
  66682c:	mov    rsi,QWORD PTR [rsp+0x18]
  666831:	vpxor  xmm8,xmm8,XMMWORD PTR [rsi+rbx*1]
  666836:	mov    rsi,QWORD PTR [rsp+0xb8]
  66683e:	vpxor  xmm5,xmm5,XMMWORD PTR [rsi+rbp*1]
  666843:	cmp    r15,0x3
  666847:	je     6668b0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x17e0>
  666849:	movzx  esi,BYTE PTR [r11+rax*1+0xc]
  66684f:	shl    esi,0x4
  666852:	movzx  r9d,BYTE PTR [r11+rax*1+0xd]
  666858:	shl    r9d,0x4
  66685c:	movzx  ebx,BYTE PTR [r11+rax*1+0xe]
  666862:	shl    ebx,0x4
  666865:	movzx  ebp,BYTE PTR [r11+rax*1+0xf]
  66686b:	shl    ebp,0x4
  66686e:	mov    rdi,QWORD PTR [rsp+0xa8]
  666876:	vpxor  xmm6,xmm6,XMMWORD PTR [rdi+rsi*1]
  66687b:	mov    rsi,QWORD PTR [rsp+0xa0]
  666883:	vpxor  xmm7,xmm7,XMMWORD PTR [rsi+r9*1]
  666889:	mov    rsi,QWORD PTR [rsp+0x98]
  666891:	vpxor  xmm8,xmm8,XMMWORD PTR [rsi+rbx*1]
  666896:	mov    rsi,QWORD PTR [rsp+0x90]
  66689e:	vpxor  xmm5,xmm5,XMMWORD PTR [rsi+rbp*1]
  6668a3:	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  6668b0:	vpxor  xmm9,xmm9,xmm9
  6668b5:	cmp    r15,0x1
  6668b9:	jne    6668f0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1820>
  6668bb:	xor    esi,esi
  6668bd:	vpxor  xmm10,xmm10,xmm10
  6668c2:	vpxor  xmm11,xmm11,xmm11
  6668c7:	vpxor  xmm12,xmm12,xmm12
  6668cc:	jmp    6669cd <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x18fd>
  6668d1:	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  6668e0:	vpxor  xmm5,xmm5,xmm5
  6668e4:	vpxor  xmm9,xmm9,xmm9
  6668e9:	jmp    666a47 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1977>
  6668ee:	xchg   ax,ax
  6668f0:	mov    r9,QWORD PTR [rsp+0x50]
  6668f5:	xor    esi,esi
  6668f7:	vpxor  xmm10,xmm10,xmm10
  6668fc:	vpxor  xmm11,xmm11,xmm11
  666901:	vpxor  xmm12,xmm12,xmm12
  666906:	cs nop WORD PTR [rax+rax*1+0x0]
  666910:	movzx  edi,BYTE PTR [r13+rsi*4-0x7]
  666916:	shl    edi,0x4
  666919:	movzx  ebx,BYTE PTR [r13+rsi*4-0x6]
  66691f:	shl    ebx,0x4
  666922:	movzx  r14d,BYTE PTR [r13+rsi*4-0x5]
  666928:	shl    r14d,0x4
  66692c:	movzx  ebp,BYTE PTR [r13+rsi*4-0x4]
  666932:	vmovdqa xmm13,XMMWORD PTR [r9+rdi*1-0x7000]
  66693c:	vmovdqa xmm14,XMMWORD PTR [r9+rbx*1-0x6000]
  666946:	shl    ebp,0x4
  666949:	vmovdqa xmm15,XMMWORD PTR [r9+r14*1-0x5000]
  666953:	vmovdqa64 xmm16,XMMWORD PTR [r9+rbp*1-0x4000]
  66695e:	movzx  edi,BYTE PTR [r13+rsi*4-0x3]
  666964:	movzx  ebx,BYTE PTR [r13+rsi*4-0x2]
  66696a:	movzx  r14d,BYTE PTR [r13+rsi*4-0x1]
  666970:	movzx  ebp,BYTE PTR [r13+rsi*4+0x0]
  666976:	add    rsi,0x2
  66697a:	shl    edi,0x4
  66697d:	shl    ebx,0x4
  666980:	shl    r14d,0x4
  666984:	shl    ebp,0x4
  666987:	vpternlogq xmm10,xmm13,XMMWORD PTR [r9+rdi*1-0x3000],0x96
  666993:	vpternlogq xmm9,xmm14,XMMWORD PTR [r9+rbx*1-0x2000],0x96
  66699f:	vpternlogq xmm11,xmm15,XMMWORD PTR [r9+r14*1-0x1000],0x96
  6669ab:	vpternlogq xmm12,xmm16,XMMWORD PTR [r9+rbp*1],0x96
  6669b3:	add    r9,0x8000
  6669ba:	cmp    r10,rsi
  6669bd:	jne    666910 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1840>
  6669c3:	test   BYTE PTR [rsp+0x150],0x1
  6669cb:	je     666a2b <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x195b>
  6669cd:	mov    rdi,QWORD PTR [rsp+0xc0]
  6669d5:	lea    r9,[rdi+rax*1]
  6669d9:	mov    rbx,rsi
  6669dc:	shl    rbx,0xe
  6669e0:	add    rbx,QWORD PTR [rsp+0x8]
  6669e5:	movzx  ebp,BYTE PTR [r9+rsi*4]
  6669ea:	shl    ebp,0x4
  6669ed:	movzx  r14d,BYTE PTR [r9+rsi*4+0x1]
  6669f3:	shl    r14d,0x4
  6669f7:	movzx  edi,BYTE PTR [r9+rsi*4+0x2]
  6669fd:	shl    edi,0x4
  666a00:	movzx  esi,BYTE PTR [r9+rsi*4+0x3]
  666a06:	shl    esi,0x4
  666a09:	vpxor  xmm10,xmm10,XMMWORD PTR [rbp+rbx*1+0x0]
  666a0f:	vpxor  xmm9,xmm9,XMMWORD PTR [r14+rbx*1+0x1000]
  666a19:	vpxor  xmm11,xmm11,XMMWORD PTR [rdi+rbx*1+0x2000]
  666a22:	vpxor  xmm12,xmm12,XMMWORD PTR [rsi+rbx*1+0x3000]
  666a2b:	vpxor  xmm9,xmm9,xmm10
  666a30:	vpternlogq xmm9,xmm12,xmm11,0x96
  666a37:	vpxor  xmm6,xmm7,xmm6
  666a3b:	vpternlogq xmm5,xmm6,xmm8,0x96
  666a42:	mov    r8,QWORD PTR [rsp+0x30]
  666a47:	mov    rdi,QWORD PTR [rsp+0x38]
  666a4c:	mov    rsi,QWORD PTR [rdi+rax*1]
  666a50:	mov    rdi,QWORD PTR [rdi+rax*1+0x8]
  666a55:	vmovq  xmm6,rsi
  666a5a:	vpclmullqlqdq xmm6,xmm9,xmm6
  666a60:	vpshufd xmm7,xmm9,0xee
  666a66:	vmovq  xmm8,rdi
  666a6b:	vpclmullqlqdq xmm8,xmm7,xmm8
  666a71:	xor    rdi,rsi
  666a74:	vpxor  xmm7,xmm9,xmm7
  666a78:	vmovq  xmm9,rdi
  666a7d:	vpclmullqlqdq xmm7,xmm7,xmm9
  666a83:	vpternlogq xmm7,xmm6,xmm8,0x96
  666a8a:	vpshufd xmm9,xmm8,0xee
  666a90:	vpclmullqlqdq xmm9,xmm9,xmm3
  666a96:	vpshufd xmm10,xmm7,0xee
  666a9b:	vpxor  xmm8,xmm10,xmm8
  666aa0:	vpclmullqlqdq xmm8,xmm8,xmm3
  666aa6:	vpextrq rdi,xmm9,0x1
  666aac:	lea    rsi,[rdi+rdi*1]
  666ab0:	lea    r9,[rdi*4+0x0]
  666ab8:	xor    r9,rsi
  666abb:	mov    rbx,rdi
  666abe:	shl    rbx,0x7
  666ac2:	xor    rbx,r9
  666ac5:	vpxor  xmm6,xmm8,xmm6
  666ac9:	vmovq  rsi,xmm6
  666ace:	xor    rsi,rbx
  666ad1:	xor    rsi,rdi
  666ad4:	vpshufd xmm6,xmm6,0xee
  666ad9:	vpternlogq xmm6,xmm7,xmm9,0x96
  666ae0:	vmovq  r9,xmm6
  666ae5:	mov    rdi,QWORD PTR [rsp+0x40]
  666aea:	vmovdqa XMMWORD PTR [rdi+rax*1],xmm5
  666aef:	mov    rdi,QWORD PTR [rsp+0x48]
  666af4:	mov    QWORD PTR [rdi+rax*1],rsi
  666af8:	vmovq  QWORD PTR [rdi+rax*1+0x8],xmm6
  666afe:	test   cl,0x1
  666b01:	jne    666670 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x15a0>
  666b07:	mov    r12,rsi
  666b0a:	mov    QWORD PTR [rsp],r9
  666b0e:	vmovdqa xmm4,xmm5
  666b12:	inc    rcx
  666b15:	add    r13,0x10
  666b19:	cmp    rcx,r8
  666b1c:	jb     66671d <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x164d>
  666b22:	mov    rax,QWORD PTR [rsp+0xb0]
  666b2a:	vmovq  QWORD PTR [rax],xmm1
  666b2e:	vmovdqu YMMWORD PTR [rax+0x8],ymm2
  666b33:	vmovdqu XMMWORD PTR [rax+0x28],xmm0
  666b38:	vpextrq QWORD PTR [rax+0x38],xmm1,0x1
  666b3f:	add    rsp,0x218
  666b46:	pop    rbx
  666b47:	pop    r12
  666b49:	pop    r13
  666b4b:	pop    r14
  666b4d:	pop    r15
  666b4f:	pop    rbp
  666b50:	vzeroupper 
  666b53:	ret    
  666b54:	lea    rsi,[r10+0x10]
  666b58:	lea    rcx,[rip+0x8e461]        # 6f4fc0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x240>
  666b5f:	mov    rdi,r10
  666b62:	vzeroupper 
  666b65:	call   QWORD PTR [rip+0x917ed]        # 6f8358 <_DYNAMIC+0x340>
  666b6b:	lea    rsi,[rax+0x10]
  666b6f:	lea    rcx,[rip+0x8e95a]        # 6f54d0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x750>
  666b76:	mov    rdi,rax
  666b79:	vzeroupper 
  666b7c:	call   QWORD PTR [rip+0x917d6]        # 6f8358 <_DYNAMIC+0x340>
  666b82:	mov    eax,0x10
  666b87:	jmp    666bb1 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ae1>
  666b89:	mov    eax,0x20
  666b8e:	jmp    666bb1 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ae1>
  666b90:	mov    eax,0x30
  666b95:	jmp    666bb1 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ae1>
  666b97:	mov    eax,0x40
  666b9c:	jmp    666bb1 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ae1>
  666b9e:	mov    eax,0x50
  666ba3:	jmp    666bb1 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ae1>
  666ba5:	mov    eax,0x60
  666baa:	jmp    666bb1 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_+0x1ae1>
  666bac:	mov    eax,0x70
  666bb1:	or     r10,rax
  666bb4:	lea    rsi,[r10+0x10]
  666bb8:	lea    rcx,[rip+0x8e3e9]        # 6f4fa8 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x228>
  666bbf:	mov    rdi,r10
  666bc2:	vzeroupper 
  666bc5:	call   QWORD PTR [rip+0x9178d]        # 6f8358 <_DYNAMIC+0x340>
  666bcb:	lea    rcx,[rip+0x8e3be]        # 6f4f90 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x210>
  666bd2:	mov    rdi,QWORD PTR [rsp]
  666bd6:	mov    rsi,r11
  666bd9:	mov    rdx,rbx
  666bdc:	call   QWORD PTR [rip+0x91776]        # 6f8358 <_DYNAMIC+0x340>
  666be2:	lea    rcx,[rip+0x8e38f]        # 6f4f78 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x1f8>
  666be9:	mov    rdi,rdx
  666bec:	mov    rsi,r10
  666bef:	mov    rdx,r11
  666bf2:	call   QWORD PTR [rip+0x91760]        # 6f8358 <_DYNAMIC+0x340>
  666bf8:	lea    rcx,[rip+0x8e361]        # 6f4f60 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x1e0>
  666bff:	mov    rdi,QWORD PTR [rsp]
  666c03:	mov    rsi,r11
  666c06:	mov    rdx,rbx
  666c09:	call   QWORD PTR [rip+0x91749]        # 6f8358 <_DYNAMIC+0x340>
  666c0f:	lea    r9,[rip+0x8e8a2]        # 6f54b8 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x738>
  666c16:	lea    rsi,[rsp+0xc8]
  666c1e:	lea    rdx,[rsp+0x190]
  666c26:	xor    edi,edi
  666c28:	xor    ecx,ecx
  666c2a:	call   QWORD PTR [rip+0x917b0]        # 6f83e0 <_DYNAMIC+0x3c8>

