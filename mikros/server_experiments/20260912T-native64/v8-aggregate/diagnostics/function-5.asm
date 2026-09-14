0000000000670300 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0B5_>:
  670300:	41 56                	push   r14
  670302:	53                   	push   rbx
  670303:	50                   	push   rax
  670304:	49 89 f6             	mov    r14,rsi
  670307:	48 89 fb             	mov    rbx,rdi
  67030a:	48 8b 4e 10          	mov    rcx,QWORD PTR [rsi+0x10]
  67030e:	48 8b 02             	mov    rax,QWORD PTR [rdx]
  670311:	48 8b 7a 08          	mov    rdi,QWORD PTR [rdx+0x8]
  670315:	48 85 c9             	test   rcx,rcx
  670318:	74 36                	je     670350 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0B5_+0x50>
  67031a:	49 8b 76 08          	mov    rsi,QWORD PTR [r14+0x8]
  67031e:	48 c1 e1 04          	shl    rcx,0x4
  670322:	48 8b 52 10          	mov    rdx,QWORD PTR [rdx+0x10]
  670326:	48 c1 e2 04          	shl    rdx,0x4
  67032a:	45 31 c0             	xor    r8d,r8d
  67032d:	0f 1f 00             	nop    DWORD PTR [rax]
  670330:	4c 39 c2             	cmp    rdx,r8
  670333:	74 1b                	je     670350 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0B5_+0x50>
  670335:	c4 a1 78 10 04 06    	vmovups xmm0,XMMWORD PTR [rsi+r8*1]
  67033b:	c4 a1 78 57 04 07    	vxorps xmm0,xmm0,XMMWORD PTR [rdi+r8*1]
  670341:	c4 a1 78 11 04 06    	vmovups XMMWORD PTR [rsi+r8*1],xmm0
  670347:	49 83 c0 10          	add    r8,0x10
  67034b:	4c 39 c1             	cmp    rcx,r8
  67034e:	75 e0                	jne    670330 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0B5_+0x30>
  670350:	48 85 c0             	test   rax,rax
  670353:	74 06                	je     67035b <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0B5_+0x5b>
  670355:	ff 15 3d 70 09 00    	call   QWORD PTR [rip+0x9703d]        # 707398 <free@GLIBC_2.2.5>
  67035b:	49 8b 46 10          	mov    rax,QWORD PTR [r14+0x10]
  67035f:	48 89 43 10          	mov    QWORD PTR [rbx+0x10],rax
  670363:	c4 c1 78 10 06       	vmovups xmm0,XMMWORD PTR [r14]
  670368:	c5 f8 11 03          	vmovups XMMWORD PTR [rbx],xmm0
  67036c:	48 83 c4 08          	add    rsp,0x8
  670370:	5b                   	pop    rbx
  670371:	41 5e                	pop    r14
  670373:	c3                   	ret    
  670374:	cc                   	int3   
  670375:	cc                   	int3   
  670376:	cc                   	int3   
  670377:	cc                   	int3   
  670378:	cc                   	int3   
  670379:	cc                   	int3   
  67037a:	cc                   	int3   
  67037b:	cc                   	int3   
  67037c:	cc                   	int3   
  67037d:	cc                   	int3   
  67037e:	cc                   	int3   
  67037f:	cc                   	int3   

