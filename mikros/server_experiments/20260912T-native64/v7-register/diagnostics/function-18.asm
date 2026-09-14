0000000000671080 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project>:
  671080:	push   rbp
  671081:	push   r15
  671083:	push   r14
  671085:	push   r13
  671087:	push   r12
  671089:	push   rbx
  67108a:	sub    rsp,0x38
  67108e:	mov    QWORD PTR [rsp],r8
  671092:	mov    QWORD PTR [rsp+0x28],rcx
  671097:	mov    rax,r9
  67109a:	shl    rax,0x4
  67109e:	mov    QWORD PTR [rsp+0x30],rax
  6710a3:	cmp    rcx,rax
  6710a6:	jne    671cb6 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xc36>
  6710ac:	mov    QWORD PTR [rsp+0x20],rdi
  6710b1:	mov    r8,r9
  6710b4:	shr    r8,0x2
  6710b8:	movabs rdi,0x1ffffffffffff
  6710c2:	je     6717b1 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x731>
  6710c8:	mov    QWORD PTR [rsp+0x8],r9
  6710cd:	mov    r15,QWORD PTR [rsi+0x10]
  6710d1:	shr    r15,0x2
  6710d5:	and    r15,rdi
  6710d8:	mov    QWORD PTR [rsp+0x18],rdx
  6710dd:	mov    QWORD PTR [rsp+0x10],rsi
  6710e2:	je     6717c2 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x742>
  6710e8:	mov    r12,QWORD PTR [rsi+0x8]
  6710ec:	lea    r13,[r12+0x1000]
  6710f4:	lea    rbp,[r12+0x2000]
  6710fc:	lea    rbx,[r12+0x3000]
  671104:	shl    r8,0x6
  671108:	vpxor  xmm0,xmm0,xmm0
  67110c:	vpxor  xmm1,xmm1,xmm1
  671110:	vpxor  xmm2,xmm2,xmm2
  671114:	lea    r9,[rdx+0x3f]
  671118:	xor    r11d,r11d
  67111b:	jmp    6711a6 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x126>
  671120:	vpxorq xmm15,xmm16,xmm15
  671126:	vpternlogq xmm15,xmm18,xmm17,0x96
  67112d:	vpxor  xmm4,xmm5,xmm4
  671131:	vpternlogq xmm3,xmm4,xmm6,0x96
  671138:	vpxor  xmm4,xmm9,xmm8
  67113d:	vpternlogq xmm7,xmm4,xmm10,0x96
  671144:	vpxor  xmm4,xmm13,xmm12
  671149:	vpternlogq xmm11,xmm4,xmm14,0x96
  671150:	vinserti128 ymm4,ymm11,xmm15,0x1
  671156:	vinserti128 ymm3,ymm3,xmm7,0x1
  67115c:	vinserti64x4 zmm3,zmm3,ymm4,0x1
  671163:	mov    rdx,QWORD PTR [rsp]
  671167:	vmovdqu64 zmm4,ZMMWORD PTR [rdx+rax*1]
  67116e:	vpclmullqlqdq zmm5,zmm3,zmm4
  671175:	vpxorq zmm0,zmm5,zmm0
  67117b:	vpclmulhqhqdq zmm5,zmm3,zmm4
  671182:	vpxorq zmm1,zmm5,zmm1
  671188:	vpclmulhqlqdq zmm5,zmm3,zmm4
  67118f:	vpclmullqhqdq zmm3,zmm3,zmm4
  671196:	vpternlogq zmm2,zmm5,zmm3,0x96
  67119d:	cmp    r8,r11
  6711a0:	je     671852 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x7d2>
  6711a6:	mov    rax,r11
  6711a9:	lea    r10,[r11+0x10]
  6711ad:	cmp    r10,rcx
  6711b0:	ja     671c4c <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xbcc>
  6711b6:	movzx  r11d,BYTE PTR [r9+rax*1-0x3f]
  6711bc:	shl    r11d,0x4
  6711c0:	movzx  edx,BYTE PTR [r9+rax*1-0x3e]
  6711c6:	shl    edx,0x4
  6711c9:	movzx  edi,BYTE PTR [r9+rax*1-0x3d]
  6711cf:	shl    edi,0x4
  6711d2:	movzx  esi,BYTE PTR [r9+rax*1-0x3c]
  6711d8:	vmovdqa xmm4,XMMWORD PTR [r12+r11*1]
  6711de:	vmovdqa xmm5,XMMWORD PTR [r13+rdx*1+0x0]
  6711e5:	shl    esi,0x4
  6711e8:	vmovdqa xmm6,XMMWORD PTR [rbp+rdi*1+0x0]
  6711ee:	vmovdqa xmm3,XMMWORD PTR [rbx+rsi*1]
  6711f3:	cmp    r15,0x1
  6711f7:	je     671320 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x2a0>
  6711fd:	movzx  edx,BYTE PTR [r9+rax*1-0x3b]
  671203:	shl    edx,0x4
  671206:	movzx  esi,BYTE PTR [r9+rax*1-0x3a]
  67120c:	shl    esi,0x4
  67120f:	movzx  edi,BYTE PTR [r9+rax*1-0x39]
  671215:	shl    edi,0x4
  671218:	movzx  r11d,BYTE PTR [r9+rax*1-0x38]
  67121e:	lea    r14,[r12+0x4000]
  671226:	vpxor  xmm4,xmm4,XMMWORD PTR [r14+rdx*1]
  67122c:	lea    rdx,[r12+0x5000]
  671234:	vpxor  xmm5,xmm5,XMMWORD PTR [rdx+rsi*1]
  671239:	shl    r11d,0x4
  67123d:	lea    rdx,[r12+0x6000]
  671245:	vpxor  xmm6,xmm6,XMMWORD PTR [rdx+rdi*1]
  67124a:	lea    rdx,[r12+0x7000]
  671252:	vpxor  xmm3,xmm3,XMMWORD PTR [rdx+r11*1]
  671258:	cmp    r15,0x2
  67125c:	je     671320 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x2a0>
  671262:	movzx  edx,BYTE PTR [r9+rax*1-0x37]
  671268:	shl    edx,0x4
  67126b:	movzx  esi,BYTE PTR [r9+rax*1-0x36]
  671271:	shl    esi,0x4
  671274:	movzx  edi,BYTE PTR [r9+rax*1-0x35]
  67127a:	shl    edi,0x4
  67127d:	movzx  r11d,BYTE PTR [r9+rax*1-0x34]
  671283:	lea    r14,[r12+0x8000]
  67128b:	vpxor  xmm4,xmm4,XMMWORD PTR [r14+rdx*1]
  671291:	lea    rdx,[r12+0x9000]
  671299:	vpxor  xmm5,xmm5,XMMWORD PTR [rdx+rsi*1]
  67129e:	shl    r11d,0x4
  6712a2:	lea    rdx,[r12+0xa000]
  6712aa:	vpxor  xmm6,xmm6,XMMWORD PTR [rdx+rdi*1]
  6712af:	lea    rdx,[r12+0xb000]
  6712b7:	vpxor  xmm3,xmm3,XMMWORD PTR [rdx+r11*1]
  6712bd:	cmp    r15,0x3
  6712c1:	je     671320 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x2a0>
  6712c3:	movzx  edx,BYTE PTR [r9+rax*1-0x33]
  6712c9:	shl    edx,0x4
  6712cc:	movzx  esi,BYTE PTR [r9+rax*1-0x32]
  6712d2:	shl    esi,0x4
  6712d5:	movzx  edi,BYTE PTR [r9+rax*1-0x31]
  6712db:	shl    edi,0x4
  6712de:	movzx  r11d,BYTE PTR [r9+rax*1-0x30]
  6712e4:	shl    r11d,0x4
  6712e8:	lea    r14,[r12+0xc000]
  6712f0:	vpxor  xmm4,xmm4,XMMWORD PTR [r14+rdx*1]
  6712f6:	lea    rdx,[r12+0xd000]
  6712fe:	vpxor  xmm5,xmm5,XMMWORD PTR [rdx+rsi*1]
  671303:	lea    rdx,[r12+0xe000]
  67130b:	vpxor  xmm6,xmm6,XMMWORD PTR [rdx+rdi*1]
  671310:	lea    rdx,[r12+0xf000]
  671318:	vpxor  xmm3,xmm3,XMMWORD PTR [rdx+r11*1]
  67131e:	xchg   ax,ax
  671320:	lea    r11,[rax+0x20]
  671324:	cmp    r11,rcx
  671327:	ja     671c69 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xbe9>
  67132d:	movzx  edx,BYTE PTR [r9+rax*1-0x2f]
  671333:	shl    edx,0x4
  671336:	movzx  esi,BYTE PTR [r9+rax*1-0x2e]
  67133c:	shl    esi,0x4
  67133f:	movzx  edi,BYTE PTR [r9+rax*1-0x2d]
  671345:	shl    edi,0x4
  671348:	movzx  r10d,BYTE PTR [r9+rax*1-0x2c]
  67134e:	vmovdqa xmm8,XMMWORD PTR [r12+rdx*1]
  671354:	vmovdqa xmm9,XMMWORD PTR [r13+rsi*1+0x0]
  67135b:	shl    r10d,0x4
  67135f:	vmovdqa xmm10,XMMWORD PTR [rbp+rdi*1+0x0]
  671365:	vmovdqa xmm7,XMMWORD PTR [rbx+r10*1]
  67136b:	cmp    r15,0x1
  67136f:	je     6714a0 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x420>
  671375:	movzx  edx,BYTE PTR [r9+rax*1-0x2b]
  67137b:	shl    edx,0x4
  67137e:	movzx  esi,BYTE PTR [r9+rax*1-0x2a]
  671384:	shl    esi,0x4
  671387:	movzx  edi,BYTE PTR [r9+rax*1-0x29]
  67138d:	shl    edi,0x4
  671390:	movzx  r10d,BYTE PTR [r9+rax*1-0x28]
  671396:	lea    r14,[r12+0x4000]
  67139e:	vpxor  xmm8,xmm8,XMMWORD PTR [r14+rdx*1]
  6713a4:	lea    rdx,[r12+0x5000]
  6713ac:	vpxor  xmm9,xmm9,XMMWORD PTR [rdx+rsi*1]
  6713b1:	shl    r10d,0x4
  6713b5:	lea    rdx,[r12+0x6000]
  6713bd:	vpxor  xmm10,xmm10,XMMWORD PTR [rdx+rdi*1]
  6713c2:	lea    rdx,[r12+0x7000]
  6713ca:	vpxor  xmm7,xmm7,XMMWORD PTR [rdx+r10*1]
  6713d0:	cmp    r15,0x2
  6713d4:	je     6714a0 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x420>
  6713da:	movzx  edx,BYTE PTR [r9+rax*1-0x27]
  6713e0:	shl    edx,0x4
  6713e3:	movzx  esi,BYTE PTR [r9+rax*1-0x26]
  6713e9:	shl    esi,0x4
  6713ec:	movzx  edi,BYTE PTR [r9+rax*1-0x25]
  6713f2:	shl    edi,0x4
  6713f5:	movzx  r10d,BYTE PTR [r9+rax*1-0x24]
  6713fb:	lea    r14,[r12+0x8000]
  671403:	vpxor  xmm8,xmm8,XMMWORD PTR [r14+rdx*1]
  671409:	lea    rdx,[r12+0x9000]
  671411:	vpxor  xmm9,xmm9,XMMWORD PTR [rdx+rsi*1]
  671416:	shl    r10d,0x4
  67141a:	lea    rdx,[r12+0xa000]
  671422:	vpxor  xmm10,xmm10,XMMWORD PTR [rdx+rdi*1]
  671427:	lea    rdx,[r12+0xb000]
  67142f:	vpxor  xmm7,xmm7,XMMWORD PTR [rdx+r10*1]
  671435:	cmp    r15,0x3
  671439:	je     6714a0 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x420>
  67143b:	movzx  edx,BYTE PTR [r9+rax*1-0x23]
  671441:	shl    edx,0x4
  671444:	movzx  esi,BYTE PTR [r9+rax*1-0x22]
  67144a:	shl    esi,0x4
  67144d:	movzx  edi,BYTE PTR [r9+rax*1-0x21]
  671453:	shl    edi,0x4
  671456:	movzx  r10d,BYTE PTR [r9+rax*1-0x20]
  67145c:	shl    r10d,0x4
  671460:	lea    r14,[r12+0xc000]
  671468:	vpxor  xmm8,xmm8,XMMWORD PTR [r14+rdx*1]
  67146e:	lea    rdx,[r12+0xd000]
  671476:	vpxor  xmm9,xmm9,XMMWORD PTR [rdx+rsi*1]
  67147b:	lea    rdx,[r12+0xe000]
  671483:	vpxor  xmm10,xmm10,XMMWORD PTR [rdx+rdi*1]
  671488:	lea    rdx,[r12+0xf000]
  671490:	vpxor  xmm7,xmm7,XMMWORD PTR [rdx+r10*1]
  671496:	cs nop WORD PTR [rax+rax*1+0x0]
  6714a0:	lea    r10,[rax+0x30]
  6714a4:	cmp    r10,rcx
  6714a7:	ja     671c76 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xbf6>
  6714ad:	movzx  edx,BYTE PTR [r9+rax*1-0x1f]
  6714b3:	shl    edx,0x4
  6714b6:	movzx  esi,BYTE PTR [r9+rax*1-0x1e]
  6714bc:	shl    esi,0x4
  6714bf:	movzx  edi,BYTE PTR [r9+rax*1-0x1d]
  6714c5:	shl    edi,0x4
  6714c8:	movzx  r11d,BYTE PTR [r9+rax*1-0x1c]
  6714ce:	vmovdqa xmm12,XMMWORD PTR [r12+rdx*1]
  6714d4:	vmovdqa xmm13,XMMWORD PTR [r13+rsi*1+0x0]
  6714db:	shl    r11d,0x4
  6714df:	vmovdqa xmm14,XMMWORD PTR [rbp+rdi*1+0x0]
  6714e5:	vmovdqa xmm11,XMMWORD PTR [rbx+r11*1]
  6714eb:	cmp    r15,0x1
  6714ef:	je     671620 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x5a0>
  6714f5:	movzx  edx,BYTE PTR [r9+rax*1-0x1b]
  6714fb:	shl    edx,0x4
  6714fe:	movzx  esi,BYTE PTR [r9+rax*1-0x1a]
  671504:	shl    esi,0x4
  671507:	movzx  edi,BYTE PTR [r9+rax*1-0x19]
  67150d:	shl    edi,0x4
  671510:	movzx  r11d,BYTE PTR [r9+rax*1-0x18]
  671516:	lea    r14,[r12+0x4000]
  67151e:	vpxor  xmm12,xmm12,XMMWORD PTR [r14+rdx*1]
  671524:	lea    rdx,[r12+0x5000]
  67152c:	vpxor  xmm13,xmm13,XMMWORD PTR [rdx+rsi*1]
  671531:	shl    r11d,0x4
  671535:	lea    rdx,[r12+0x6000]
  67153d:	vpxor  xmm14,xmm14,XMMWORD PTR [rdx+rdi*1]
  671542:	lea    rdx,[r12+0x7000]
  67154a:	vpxor  xmm11,xmm11,XMMWORD PTR [rdx+r11*1]
  671550:	cmp    r15,0x2
  671554:	je     671620 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x5a0>
  67155a:	movzx  edx,BYTE PTR [r9+rax*1-0x17]
  671560:	shl    edx,0x4
  671563:	movzx  esi,BYTE PTR [r9+rax*1-0x16]
  671569:	shl    esi,0x4
  67156c:	movzx  edi,BYTE PTR [r9+rax*1-0x15]
  671572:	shl    edi,0x4
  671575:	movzx  r11d,BYTE PTR [r9+rax*1-0x14]
  67157b:	lea    r14,[r12+0x8000]
  671583:	vpxor  xmm12,xmm12,XMMWORD PTR [r14+rdx*1]
  671589:	lea    rdx,[r12+0x9000]
  671591:	vpxor  xmm13,xmm13,XMMWORD PTR [rdx+rsi*1]
  671596:	shl    r11d,0x4
  67159a:	lea    rdx,[r12+0xa000]
  6715a2:	vpxor  xmm14,xmm14,XMMWORD PTR [rdx+rdi*1]
  6715a7:	lea    rdx,[r12+0xb000]
  6715af:	vpxor  xmm11,xmm11,XMMWORD PTR [rdx+r11*1]
  6715b5:	cmp    r15,0x3
  6715b9:	je     671620 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x5a0>
  6715bb:	movzx  edx,BYTE PTR [r9+rax*1-0x13]
  6715c1:	shl    edx,0x4
  6715c4:	movzx  esi,BYTE PTR [r9+rax*1-0x12]
  6715ca:	shl    esi,0x4
  6715cd:	movzx  edi,BYTE PTR [r9+rax*1-0x11]
  6715d3:	shl    edi,0x4
  6715d6:	movzx  r11d,BYTE PTR [r9+rax*1-0x10]
  6715dc:	shl    r11d,0x4
  6715e0:	lea    r14,[r12+0xc000]
  6715e8:	vpxor  xmm12,xmm12,XMMWORD PTR [r14+rdx*1]
  6715ee:	lea    rdx,[r12+0xd000]
  6715f6:	vpxor  xmm13,xmm13,XMMWORD PTR [rdx+rsi*1]
  6715fb:	lea    rdx,[r12+0xe000]
  671603:	vpxor  xmm14,xmm14,XMMWORD PTR [rdx+rdi*1]
  671608:	lea    rdx,[r12+0xf000]
  671610:	vpxor  xmm11,xmm11,XMMWORD PTR [rdx+r11*1]
  671616:	cs nop WORD PTR [rax+rax*1+0x0]
  671620:	lea    r11,[rax+0x40]
  671624:	cmp    r11,rcx
  671627:	ja     671c96 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xc16>
  67162d:	movzx  edx,BYTE PTR [r9+rax*1-0xf]
  671633:	shl    edx,0x4
  671636:	movzx  esi,BYTE PTR [r9+rax*1-0xe]
  67163c:	shl    esi,0x4
  67163f:	movzx  edi,BYTE PTR [r9+rax*1-0xd]
  671645:	shl    edi,0x4
  671648:	movzx  r10d,BYTE PTR [r9+rax*1-0xc]
  67164e:	vmovdqa xmm15,XMMWORD PTR [r12+rdx*1]
  671654:	vmovdqa64 xmm16,XMMWORD PTR [r13+rsi*1+0x0]
  67165c:	shl    r10d,0x4
  671660:	vmovdqa64 xmm17,XMMWORD PTR [rbp+rdi*1+0x0]
  671668:	vmovdqa64 xmm18,XMMWORD PTR [rbx+r10*1]
  67166f:	cmp    r15,0x1
  671673:	je     671120 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xa0>
  671679:	movzx  edx,BYTE PTR [r9+rax*1-0xb]
  67167f:	shl    edx,0x4
  671682:	movzx  esi,BYTE PTR [r9+rax*1-0xa]
  671688:	shl    esi,0x4
  67168b:	movzx  edi,BYTE PTR [r9+rax*1-0x9]
  671691:	shl    edi,0x4
  671694:	movzx  r10d,BYTE PTR [r9+rax*1-0x8]
  67169a:	lea    r14,[r12+0x4000]
  6716a2:	vpxor  xmm15,xmm15,XMMWORD PTR [r14+rdx*1]
  6716a8:	lea    rdx,[r12+0x5000]
  6716b0:	vpxorq xmm16,xmm16,XMMWORD PTR [rdx+rsi*1]
  6716b7:	shl    r10d,0x4
  6716bb:	lea    rdx,[r12+0x6000]
  6716c3:	vpxorq xmm17,xmm17,XMMWORD PTR [rdx+rdi*1]
  6716ca:	lea    rdx,[r12+0x7000]
  6716d2:	vpxorq xmm18,xmm18,XMMWORD PTR [rdx+r10*1]
  6716d9:	cmp    r15,0x2
  6716dd:	je     671120 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xa0>
  6716e3:	movzx  edx,BYTE PTR [r9+rax*1-0x7]
  6716e9:	shl    edx,0x4
  6716ec:	movzx  esi,BYTE PTR [r9+rax*1-0x6]
  6716f2:	shl    esi,0x4
  6716f5:	movzx  edi,BYTE PTR [r9+rax*1-0x5]
  6716fb:	shl    edi,0x4
  6716fe:	movzx  r10d,BYTE PTR [r9+rax*1-0x4]
  671704:	lea    r14,[r12+0x8000]
  67170c:	vpxor  xmm15,xmm15,XMMWORD PTR [r14+rdx*1]
  671712:	lea    rdx,[r12+0x9000]
  67171a:	vpxorq xmm16,xmm16,XMMWORD PTR [rdx+rsi*1]
  671721:	shl    r10d,0x4
  671725:	lea    rdx,[r12+0xa000]
  67172d:	vpxorq xmm17,xmm17,XMMWORD PTR [rdx+rdi*1]
  671734:	lea    rdx,[r12+0xb000]
  67173c:	vpxorq xmm18,xmm18,XMMWORD PTR [rdx+r10*1]
  671743:	cmp    r15,0x3
  671747:	je     671120 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xa0>
  67174d:	movzx  edx,BYTE PTR [r9+rax*1-0x3]
  671753:	shl    edx,0x4
  671756:	movzx  esi,BYTE PTR [r9+rax*1-0x2]
  67175c:	shl    esi,0x4
  67175f:	movzx  edi,BYTE PTR [r9+rax*1-0x1]
  671765:	shl    edi,0x4
  671768:	movzx  r10d,BYTE PTR [r9+rax*1]
  67176d:	shl    r10d,0x4
  671771:	lea    r14,[r12+0xc000]
  671779:	vpxor  xmm15,xmm15,XMMWORD PTR [r14+rdx*1]
  67177f:	lea    rdx,[r12+0xd000]
  671787:	vpxorq xmm16,xmm16,XMMWORD PTR [rdx+rsi*1]
  67178e:	lea    rdx,[r12+0xe000]
  671796:	vpxorq xmm17,xmm17,XMMWORD PTR [rdx+rdi*1]
  67179d:	lea    rdx,[r12+0xf000]
  6717a5:	vpxorq xmm18,xmm18,XMMWORD PTR [rdx+r10*1]
  6717ac:	jmp    671120 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xa0>
  6717b1:	vpxor  xmm2,xmm2,xmm2
  6717b5:	vpxor  xmm1,xmm1,xmm1
  6717b9:	vpxor  xmm0,xmm0,xmm0
  6717bd:	jmp    67186b <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x7eb>
  6717c2:	shl    r8,0x6
  6717c6:	vpxor  xmm3,xmm3,xmm3
  6717ca:	xor    eax,eax
  6717cc:	vpxor  xmm0,xmm0,xmm0
  6717d0:	vpxor  xmm1,xmm1,xmm1
  6717d4:	vpxor  xmm2,xmm2,xmm2
  6717d8:	mov    rsi,QWORD PTR [rsp]
  6717dc:	nop    DWORD PTR [rax+0x0]
  6717e0:	lea    r10,[rax+0x10]
  6717e4:	cmp    r10,rcx
  6717e7:	ja     671c4c <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xbcc>
  6717ed:	lea    r11,[rax+0x20]
  6717f1:	cmp    r11,rcx
  6717f4:	ja     671c69 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xbe9>
  6717fa:	lea    r10,[rax+0x30]
  6717fe:	cmp    r10,rcx
  671801:	ja     671c76 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xbf6>
  671807:	lea    rdx,[rax+0x40]
  67180b:	cmp    rdx,rcx
  67180e:	ja     671c96 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xc16>
  671814:	vmovdqu64 zmm4,ZMMWORD PTR [rsi+rax*1]
  67181b:	vpclmullqlqdq zmm5,zmm3,zmm4
  671822:	vpxorq zmm0,zmm5,zmm0
  671828:	vpclmulhqhqdq zmm5,zmm3,zmm4
  67182f:	vpxorq zmm1,zmm5,zmm1
  671835:	vpclmulhqlqdq zmm5,zmm3,zmm4
  67183c:	vpclmullqhqdq zmm4,zmm3,zmm4
  671843:	vpternlogq zmm2,zmm5,zmm4,0x96
  67184a:	mov    rax,rdx
  67184d:	cmp    r8,rdx
  671850:	jne    6717e0 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x760>
  671852:	mov    rdx,QWORD PTR [rsp+0x18]
  671857:	mov    rsi,QWORD PTR [rsp+0x10]
  67185c:	mov    r9,QWORD PTR [rsp+0x8]
  671861:	movabs rdi,0x1ffffffffffff
  67186b:	movabs rax,0x7fffffffffffffc
  671875:	and    rax,r9
  671878:	shl    rax,0x4
  67187c:	mov    r8,rcx
  67187f:	sub    r8,rax
  671882:	jb     671cd1 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xc51>
  671888:	vextracti32x4 xmm3,zmm2,0x3
  67188f:	vextracti32x4 xmm4,zmm2,0x2
  671896:	vpternlogd xmm4,xmm2,xmm3,0x96
  67189d:	vextracti128 xmm2,ymm2,0x1
  6718a3:	vextracti32x4 xmm3,zmm1,0x3
  6718aa:	vextracti32x4 xmm5,zmm1,0x2
  6718b1:	vpxor  xmm3,xmm3,xmm5
  6718b5:	vextracti128 xmm5,ymm1,0x1
  6718bb:	vpternlogd xmm5,xmm3,xmm1,0x96
  6718c2:	vextracti32x4 xmm1,zmm0,0x3
  6718c9:	vextracti32x4 xmm3,zmm0,0x2
  6718d0:	vpxor  xmm1,xmm1,xmm3
  6718d4:	vextracti128 xmm3,ymm0,0x1
  6718da:	vpternlogd xmm3,xmm1,xmm0,0x96
  6718e1:	vpalignr xmm0,xmm5,xmm3,0x8
  6718e7:	vpternlogq xmm0,xmm4,xmm2,0x96
  6718ee:	and    r9d,0x3
  6718f2:	shr    r8,0x4
  6718f6:	cmp    r9,r8
  6718f9:	cmovb  r8,r9
  6718fd:	vpalignr xmm1,xmm3,xmm5,0x8
  671903:	test   r8,r8
  671906:	je     671c25 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xba5>
  67190c:	mov    r9,r8
  67190f:	mov    rcx,QWORD PTR [rsi+0x10]
  671913:	shr    rcx,0x2
  671917:	and    rcx,rdi
  67191a:	je     671b78 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xaf8>
  671920:	mov    rsi,QWORD PTR [rsi+0x8]
  671924:	mov    rdi,QWORD PTR [rsp]
  671928:	lea    r8,[rax+rdi*1+0x8]
  67192d:	lea    rax,[rax+rdx*1+0xf]
  671932:	mov    rdx,r9
  671935:	shl    rdx,0x4
  671939:	xor    r13d,r13d
  67193c:	mov    QWORD PTR [rsp],rdx
  671940:	jmp    6719bf <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x93f>
  671942:	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  671950:	mov    rdx,QWORD PTR [rsp]
  671954:	xor    rbx,r9
  671957:	xor    r15,rdi
  67195a:	xor    r12,r14
  67195d:	xor    r12,r15
  671960:	xor    rbp,r11
  671963:	xor    rbp,rbx
  671966:	vmovq  xmm2,r12
  67196b:	vmovq  xmm3,QWORD PTR [r8+r13*1-0x8]
  671972:	vpclmullqlqdq xmm4,xmm2,xmm3
  671978:	vmovq  xmm5,QWORD PTR [r8+r13*1]
  67197e:	vpclmullqlqdq xmm2,xmm2,xmm5
  671984:	vmovq  xmm6,rbp
  671989:	vpclmullqlqdq xmm3,xmm6,xmm3
  67198f:	vpxor  xmm2,xmm3,xmm2
  671993:	vpclmullqlqdq xmm3,xmm6,xmm5
  671999:	vpunpckhqdq xmm5,xmm4,xmm2
  67199d:	vpunpcklqdq xmm2,xmm2,xmm3
  6719a1:	vpternlogq xmm0,xmm5,xmm2,0x96
  6719a8:	vpalignr xmm2,xmm4,xmm3,0x8
  6719ae:	vpxor  xmm1,xmm2,xmm1
  6719b2:	add    r13,0x10
  6719b6:	cmp    rdx,r13
  6719b9:	je     671c25 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xba5>
  6719bf:	movzx  edi,BYTE PTR [rax+r13*1-0xf]
  6719c5:	shl    edi,0x4
  6719c8:	movzx  r9d,BYTE PTR [rax+r13*1-0xe]
  6719ce:	shl    r9d,0x4
  6719d2:	movzx  ebx,BYTE PTR [rax+r13*1-0xd]
  6719d8:	shl    ebx,0x4
  6719db:	movzx  r10d,BYTE PTR [rax+r13*1-0xc]
  6719e1:	shl    r10d,0x4
  6719e5:	mov    r12,QWORD PTR [rsi+rdi*1]
  6719e9:	mov    rbp,QWORD PTR [rsi+rdi*1+0x8]
  6719ee:	lea    rdi,[rsi+0x1000]
  6719f5:	mov    r14,QWORD PTR [rdi+r9*1]
  6719f9:	mov    r11,QWORD PTR [rdi+r9*1+0x8]
  6719fe:	lea    rdi,[rsi+0x2000]
  671a05:	mov    r15,QWORD PTR [rdi+rbx*1]
  671a09:	mov    rbx,QWORD PTR [rdi+rbx*1+0x8]
  671a0e:	lea    r9,[rsi+0x3000]
  671a15:	mov    rdi,QWORD PTR [r9+r10*1]
  671a19:	mov    r9,QWORD PTR [r9+r10*1+0x8]
  671a1e:	cmp    rcx,0x1
  671a22:	je     671954 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x8d4>
  671a28:	movzx  r10d,BYTE PTR [rax+r13*1-0xb]
  671a2e:	shl    r10d,0x4
  671a32:	lea    rdx,[rsi+0x4000]
  671a39:	xor    r12,QWORD PTR [rdx+r10*1]
  671a3d:	xor    rbp,QWORD PTR [rdx+r10*1+0x8]
  671a42:	movzx  r10d,BYTE PTR [rax+r13*1-0xa]
  671a48:	shl    r10d,0x4
  671a4c:	lea    rdx,[rsi+0x5000]
  671a53:	xor    r14,QWORD PTR [rdx+r10*1]
  671a57:	xor    r11,QWORD PTR [rdx+r10*1+0x8]
  671a5c:	movzx  r10d,BYTE PTR [rax+r13*1-0x9]
  671a62:	shl    r10d,0x4
  671a66:	lea    rdx,[rsi+0x6000]
  671a6d:	xor    r15,QWORD PTR [rdx+r10*1]
  671a71:	xor    rbx,QWORD PTR [rdx+r10*1+0x8]
  671a76:	movzx  r10d,BYTE PTR [rax+r13*1-0x8]
  671a7c:	shl    r10d,0x4
  671a80:	lea    rdx,[rsi+0x7000]
  671a87:	xor    rdi,QWORD PTR [rdx+r10*1]
  671a8b:	xor    r9,QWORD PTR [rdx+r10*1+0x8]
  671a90:	cmp    rcx,0x2
  671a94:	je     671950 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x8d0>
  671a9a:	movzx  r10d,BYTE PTR [rax+r13*1-0x7]
  671aa0:	shl    r10d,0x4
  671aa4:	lea    rdx,[rsi+0x8000]
  671aab:	xor    r12,QWORD PTR [rdx+r10*1]
  671aaf:	xor    rbp,QWORD PTR [rdx+r10*1+0x8]
  671ab4:	movzx  r10d,BYTE PTR [rax+r13*1-0x6]
  671aba:	shl    r10d,0x4
  671abe:	lea    rdx,[rsi+0x9000]
  671ac5:	xor    r14,QWORD PTR [rdx+r10*1]
  671ac9:	xor    r11,QWORD PTR [rdx+r10*1+0x8]
  671ace:	movzx  r10d,BYTE PTR [rax+r13*1-0x5]
  671ad4:	shl    r10d,0x4
  671ad8:	lea    rdx,[rsi+0xa000]
  671adf:	xor    r15,QWORD PTR [rdx+r10*1]
  671ae3:	xor    rbx,QWORD PTR [rdx+r10*1+0x8]
  671ae8:	movzx  r10d,BYTE PTR [rax+r13*1-0x4]
  671aee:	shl    r10d,0x4
  671af2:	lea    rdx,[rsi+0xb000]
  671af9:	xor    rdi,QWORD PTR [rdx+r10*1]
  671afd:	xor    r9,QWORD PTR [rdx+r10*1+0x8]
  671b02:	cmp    rcx,0x3
  671b06:	je     671950 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x8d0>
  671b0c:	movzx  r10d,BYTE PTR [rax+r13*1-0x3]
  671b12:	shl    r10d,0x4
  671b16:	lea    rdx,[rsi+0xc000]
  671b1d:	xor    r12,QWORD PTR [rdx+r10*1]
  671b21:	xor    rbp,QWORD PTR [rdx+r10*1+0x8]
  671b26:	movzx  r10d,BYTE PTR [rax+r13*1-0x2]
  671b2c:	shl    r10d,0x4
  671b30:	lea    rdx,[rsi+0xd000]
  671b37:	xor    r14,QWORD PTR [rdx+r10*1]
  671b3b:	xor    r11,QWORD PTR [rdx+r10*1+0x8]
  671b40:	movzx  r10d,BYTE PTR [rax+r13*1-0x1]
  671b46:	shl    r10d,0x4
  671b4a:	lea    rdx,[rsi+0xe000]
  671b51:	xor    r15,QWORD PTR [rdx+r10*1]
  671b55:	xor    rbx,QWORD PTR [rdx+r10*1+0x8]
  671b5a:	movzx  r10d,BYTE PTR [rax+r13*1]
  671b5f:	shl    r10d,0x4
  671b63:	lea    rdx,[rsi+0xf000]
  671b6a:	xor    rdi,QWORD PTR [rdx+r10*1]
  671b6e:	xor    r9,QWORD PTR [rdx+r10*1+0x8]
  671b73:	jmp    671950 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0x8d0>
  671b78:	mov    rcx,QWORD PTR [rsp]
  671b7c:	add    rcx,rax
  671b7f:	vmovq  xmm3,QWORD PTR [rcx]
  671b83:	vpxor  xmm2,xmm2,xmm2
  671b87:	vpclmullqlqdq xmm3,xmm2,xmm3
  671b8d:	vmovq  xmm4,QWORD PTR [rcx+0x8]
  671b92:	vpclmullqlqdq xmm4,xmm2,xmm4
  671b98:	vpalignr xmm5,xmm4,xmm3,0x8
  671b9e:	vpxor  xmm5,xmm3,xmm5
  671ba2:	vpternlogq xmm0,xmm5,xmm4,0x96
  671ba9:	vpalignr xmm3,xmm3,xmm4,0x8
  671baf:	vpxor  xmm1,xmm3,xmm1
  671bb3:	cmp    r9,0x1
  671bb7:	je     671c25 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xba5>
  671bb9:	vmovq  xmm3,QWORD PTR [rcx+0x10]
  671bbe:	vpclmullqlqdq xmm3,xmm2,xmm3
  671bc4:	vmovq  xmm4,QWORD PTR [rcx+0x18]
  671bc9:	vpclmullqlqdq xmm2,xmm2,xmm4
  671bcf:	vpalignr xmm4,xmm2,xmm3,0x8
  671bd5:	vpxor  xmm4,xmm3,xmm4
  671bd9:	vpternlogq xmm0,xmm4,xmm2,0x96
  671be0:	vpalignr xmm2,xmm3,xmm2,0x8
  671be6:	vpxor  xmm1,xmm2,xmm1
  671bea:	cmp    r9,0x2
  671bee:	je     671c25 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xba5>
  671bf0:	vmovq  xmm2,QWORD PTR [rcx+0x20]
  671bf5:	vpxor  xmm3,xmm3,xmm3
  671bf9:	vpclmullqlqdq xmm2,xmm3,xmm2
  671bff:	vmovq  xmm4,QWORD PTR [rcx+0x28]
  671c04:	vpclmullqlqdq xmm3,xmm3,xmm4
  671c0a:	vpalignr xmm4,xmm3,xmm2,0x8
  671c10:	vpxor  xmm4,xmm2,xmm4
  671c14:	vpternlogq xmm0,xmm4,xmm3,0x96
  671c1b:	vpalignr xmm2,xmm2,xmm3,0x8
  671c21:	vpxor  xmm1,xmm2,xmm1
  671c25:	mov    rax,QWORD PTR [rsp+0x20]
  671c2a:	vpextrq QWORD PTR [rax],xmm1,0x1
  671c30:	vmovdqu XMMWORD PTR [rax+0x8],xmm0
  671c35:	vmovq  QWORD PTR [rax+0x18],xmm1
  671c3a:	add    rsp,0x38
  671c3e:	pop    rbx
  671c3f:	pop    r12
  671c41:	pop    r13
  671c43:	pop    r14
  671c45:	pop    r15
  671c47:	pop    rbp
  671c48:	vzeroupper 
  671c4b:	ret    
  671c4c:	lea    rsi,[rax+0x10]
  671c50:	lea    r8,[rip+0x838c1]        # 6f5518 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x798>
  671c57:	mov    rdi,rax
  671c5a:	mov    rdx,rcx
  671c5d:	mov    rcx,r8
  671c60:	vzeroupper 
  671c63:	call   QWORD PTR [rip+0x866ef]        # 6f8358 <_DYNAMIC+0x340>
  671c69:	add    rax,0x20
  671c6d:	lea    r8,[rip+0x838bc]        # 6f5530 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x7b0>
  671c74:	jmp    671ca1 <_RNvNtNtCs5Palesvl9wF_6mikros6lookup6x86_647project+0xc21>
  671c76:	add    rax,0x30
  671c7a:	lea    r8,[rip+0x838c7]        # 6f5548 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x7c8>
  671c81:	mov    rdi,r11
  671c84:	mov    rsi,rax
  671c87:	mov    rdx,rcx
  671c8a:	mov    rcx,r8
  671c8d:	vzeroupper 
  671c90:	call   QWORD PTR [rip+0x866c2]        # 6f8358 <_DYNAMIC+0x340>
  671c96:	add    rax,0x40
  671c9a:	lea    r8,[rip+0x838bf]        # 6f5560 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x7e0>
  671ca1:	mov    rdi,r10
  671ca4:	mov    rsi,rax
  671ca7:	mov    rdx,rcx
  671caa:	mov    rcx,r8
  671cad:	vzeroupper 
  671cb0:	call   QWORD PTR [rip+0x866a2]        # 6f8358 <_DYNAMIC+0x340>
  671cb6:	lea    r9,[rip+0x8382b]        # 6f54e8 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x768>
  671cbd:	lea    rsi,[rsp+0x28]
  671cc2:	lea    rdx,[rsp+0x30]
  671cc7:	xor    edi,edi
  671cc9:	xor    ecx,ecx
  671ccb:	call   QWORD PTR [rip+0x8670f]        # 6f83e0 <_DYNAMIC+0x3c8>
  671cd1:	lea    r8,[rip+0x83828]        # 6f5500 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x780>
  671cd8:	mov    rdi,rax
  671cdb:	mov    rsi,rcx
  671cde:	mov    rdx,rcx
  671ce1:	mov    rcx,r8
  671ce4:	vzeroupper 
  671ce7:	call   QWORD PTR [rip+0x8666b]        # 6f8358 <_DYNAMIC+0x340>
  671ced:	int3   
  671cee:	int3   
  671cef:	int3   

