0000000000678060 <_RNvNtCs5Palesvl9wF_6mikros10univariate9aggregate>:
  678060:	55                   	push   rbp
  678061:	41 57                	push   r15
  678063:	41 56                	push   r14
  678065:	41 55                	push   r13
  678067:	41 54                	push   r12
  678069:	53                   	push   rbx
  67806a:	48 81 ec 18 01 00 00 	sub    rsp,0x118
  678071:	48 89 54 24 18       	mov    QWORD PTR [rsp+0x18],rdx
  678076:	4c 89 44 24 20       	mov    QWORD PTR [rsp+0x20],r8
  67807b:	4c 39 c2             	cmp    rdx,r8
  67807e:	0f 85 be 01 00 00    	jne    678242 <_RNvNtCs5Palesvl9wF_6mikros10univariate9aggregate+0x1e2>
  678084:	4c 8b bc 24 50 01 00 	mov    r15,QWORD PTR [rsp+0x150]
  67808b:	00 
  67808c:	48 89 54 24 30       	mov    QWORD PTR [rsp+0x30],rdx
  678091:	4c 89 f8             	mov    rax,r15
  678094:	48 c1 e0 04          	shl    rax,0x4
  678098:	48 89 44 24 28       	mov    QWORD PTR [rsp+0x28],rax
  67809d:	48 39 c2             	cmp    rdx,rax
  6780a0:	0f 85 b7 01 00 00    	jne    67825d <_RNvNtCs5Palesvl9wF_6mikros10univariate9aggregate+0x1fd>
  6780a6:	48 89 fb             	mov    rbx,rdi
  6780a9:	8b 05 a1 2c 09 00    	mov    eax,DWORD PTR [rip+0x92ca1]        # 70ad50 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1120>
  6780af:	85 c0                	test   eax,eax
  6780b1:	0f 85 c1 01 00 00    	jne    678278 <_RNvNtCs5Palesvl9wF_6mikros10univariate9aggregate+0x218>
  6780b7:	8b 05 6b 1b 09 00    	mov    eax,DWORD PTR [rip+0x91b6b]        # 709c28 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x930>
  6780bd:	85 c0                	test   eax,eax
  6780bf:	0f 85 de 01 00 00    	jne    6782a3 <_RNvNtCs5Palesvl9wF_6mikros10univariate9aggregate+0x243>
  6780c5:	48 89 b4 24 c8 00 00 	mov    QWORD PTR [rsp+0xc8],rsi
  6780cc:	00 
  6780cd:	48 89 94 24 d0 00 00 	mov    QWORD PTR [rsp+0xd0],rdx
  6780d4:	00 
  6780d5:	48 c7 84 24 d8 00 00 	mov    QWORD PTR [rsp+0xd8],0x8000
  6780dc:	00 00 80 00 00 
  6780e1:	48 89 8c 24 e0 00 00 	mov    QWORD PTR [rsp+0xe0],rcx
  6780e8:	00 
  6780e9:	48 89 94 24 e8 00 00 	mov    QWORD PTR [rsp+0xe8],rdx
  6780f0:	00 
  6780f1:	48 c7 84 24 f0 00 00 	mov    QWORD PTR [rsp+0xf0],0x8000
  6780f8:	00 00 80 00 00 
  6780fd:	4c 89 8c 24 f8 00 00 	mov    QWORD PTR [rsp+0xf8],r9
  678104:	00 
  678105:	4c 89 bc 24 00 01 00 	mov    QWORD PTR [rsp+0x100],r15
  67810c:	00 
  67810d:	48 c7 84 24 08 01 00 	mov    QWORD PTR [rsp+0x108],0x800
  678114:	00 00 08 00 00 
  678119:	48 8d 84 24 10 01 00 	lea    rax,[rsp+0x110]
  678120:	00 
  678121:	48 8d bc 24 58 01 00 	lea    rdi,[rsp+0x158]
  678128:	00 
  678129:	48 89 bc 24 10 01 00 	mov    QWORD PTR [rsp+0x110],rdi
  678130:	00 
  678131:	48 8d ba ff 7f 00 00 	lea    rdi,[rdx+0x7fff]
  678138:	48 c1 ef 0f          	shr    rdi,0xf
  67813c:	4d 8d b7 ff 07 00 00 	lea    r14,[r15+0x7ff]
  678143:	49 c1 ee 0b          	shr    r14,0xb
  678147:	49 39 fe             	cmp    r14,rdi
  67814a:	4c 0f 43 f7          	cmovae r14,rdi
  67814e:	4c 8d 64 24 60       	lea    r12,[rsp+0x60]
  678153:	48 8d 7c 24 17       	lea    rdi,[rsp+0x17]
  678158:	48 89 7c 24 60       	mov    QWORD PTR [rsp+0x60],rdi
  67815d:	48 89 7c 24 68       	mov    QWORD PTR [rsp+0x68],rdi
  678162:	48 89 44 24 70       	mov    QWORD PTR [rsp+0x70],rax
  678167:	4c 89 74 24 78       	mov    QWORD PTR [rsp+0x78],r14
  67816c:	48 89 74 24 30       	mov    QWORD PTR [rsp+0x30],rsi
  678171:	48 89 54 24 38       	mov    QWORD PTR [rsp+0x38],rdx
  678176:	48 c7 44 24 40 00 80 	mov    QWORD PTR [rsp+0x40],0x8000
  67817d:	00 00 
  67817f:	48 89 4c 24 48       	mov    QWORD PTR [rsp+0x48],rcx
  678184:	48 89 54 24 50       	mov    QWORD PTR [rsp+0x50],rdx
  678189:	48 c7 44 24 58 00 80 	mov    QWORD PTR [rsp+0x58],0x8000
  678190:	00 00 
  678192:	48 89 b4 24 80 00 00 	mov    QWORD PTR [rsp+0x80],rsi
  678199:	00 
  67819a:	48 89 94 24 88 00 00 	mov    QWORD PTR [rsp+0x88],rdx
  6781a1:	00 
  6781a2:	48 c7 84 24 90 00 00 	mov    QWORD PTR [rsp+0x90],0x8000
  6781a9:	00 00 80 00 00 
  6781ae:	48 89 8c 24 98 00 00 	mov    QWORD PTR [rsp+0x98],rcx
  6781b5:	00 
  6781b6:	48 89 94 24 a0 00 00 	mov    QWORD PTR [rsp+0xa0],rdx
  6781bd:	00 
  6781be:	48 c7 84 24 a8 00 00 	mov    QWORD PTR [rsp+0xa8],0x8000
  6781c5:	00 00 80 00 00 
  6781ca:	4c 89 8c 24 b0 00 00 	mov    QWORD PTR [rsp+0xb0],r9
  6781d1:	00 
  6781d2:	4c 89 bc 24 b8 00 00 	mov    QWORD PTR [rsp+0xb8],r15
  6781d9:	00 
  6781da:	48 c7 84 24 c0 00 00 	mov    QWORD PTR [rsp+0xc0],0x800
  6781e1:	00 00 08 00 00 
  6781e6:	48 c7 c0 a8 ff ff ff 	mov    rax,0xffffffffffffffa8
  6781ed:	64 48 8b 00          	mov    rax,QWORD PTR fs:[rax]
  6781f1:	48 85 c0             	test   rax,rax
  6781f4:	74 08                	je     6781fe <_RNvNtCs5Palesvl9wF_6mikros10univariate9aggregate+0x19e>
  6781f6:	48 05 10 01 00 00    	add    rax,0x110
  6781fc:	eb 06                	jmp    678204 <_RNvNtCs5Palesvl9wF_6mikros10univariate9aggregate+0x1a4>
  6781fe:	ff 15 54 f5 08 00    	call   QWORD PTR [rip+0x8f554]        # 707758 <_DYNAMIC+0x5c8>
  678204:	48 8b 00             	mov    rax,QWORD PTR [rax]
  678207:	48 8b 88 08 02 00 00 	mov    rcx,QWORD PTR [rax+0x208]
  67820e:	4c 89 24 24          	mov    QWORD PTR [rsp],r12
  678212:	4c 8d 8c 24 80 00 00 	lea    r9,[rsp+0x80]
  678219:	00 
  67821a:	41 b8 01 00 00 00    	mov    r8d,0x1
  678220:	48 89 df             	mov    rdi,rbx
  678223:	4c 89 f6             	mov    rsi,r14
  678226:	31 d2                	xor    edx,edx
  678228:	e8 83 8c fe ff       	call   660eb0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_>
  67822d:	48 89 d8             	mov    rax,rbx
  678230:	48 81 c4 18 01 00 00 	add    rsp,0x118
  678237:	5b                   	pop    rbx
  678238:	41 5c                	pop    r12
  67823a:	41 5d                	pop    r13
  67823c:	41 5e                	pop    r14
  67823e:	41 5f                	pop    r15
  678240:	5d                   	pop    rbp
  678241:	c3                   	ret    
  678242:	4c 8d 0d 27 c0 08 00 	lea    r9,[rip+0x8c027]        # 704270 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x3f0>
  678249:	48 8d 74 24 18       	lea    rsi,[rsp+0x18]
  67824e:	48 8d 54 24 20       	lea    rdx,[rsp+0x20]
  678253:	31 ff                	xor    edi,edi
  678255:	31 c9                	xor    ecx,ecx
  678257:	ff 15 fb f2 08 00    	call   QWORD PTR [rip+0x8f2fb]        # 707558 <_DYNAMIC+0x3c8>
  67825d:	4c 8d 0d 24 c0 08 00 	lea    r9,[rip+0x8c024]        # 704288 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x408>
  678264:	48 8d 74 24 30       	lea    rsi,[rsp+0x30]
  678269:	48 8d 54 24 28       	lea    rdx,[rsp+0x28]
  67826e:	31 ff                	xor    edi,edi
  678270:	31 c9                	xor    ecx,ecx
  678272:	ff 15 e0 f2 08 00    	call   QWORD PTR [rip+0x8f2e0]        # 707558 <_DYNAMIC+0x3c8>
  678278:	4d 89 ce             	mov    r14,r9
  67827b:	49 89 d5             	mov    r13,rdx
  67827e:	49 89 cc             	mov    r12,rcx
  678281:	48 89 f5             	mov    rbp,rsi
  678284:	e8 3d 5e fe ff       	call   65e0c6 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  678289:	48 89 ee             	mov    rsi,rbp
  67828c:	4c 89 e1             	mov    rcx,r12
  67828f:	4c 89 ea             	mov    rdx,r13
  678292:	4d 89 f1             	mov    r9,r14
  678295:	8b 05 8d 19 09 00    	mov    eax,DWORD PTR [rip+0x9198d]        # 709c28 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x930>
  67829b:	85 c0                	test   eax,eax
  67829d:	0f 84 22 fe ff ff    	je     6780c5 <_RNvNtCs5Palesvl9wF_6mikros10univariate9aggregate+0x65>
  6782a3:	4d 89 ce             	mov    r14,r9
  6782a6:	49 89 d5             	mov    r13,rdx
  6782a9:	49 89 cc             	mov    r12,rcx
  6782ac:	48 89 f5             	mov    rbp,rsi
  6782af:	e8 c0 5d fe ff       	call   65e074 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  6782b4:	48 89 ee             	mov    rsi,rbp
  6782b7:	4c 89 e1             	mov    rcx,r12
  6782ba:	4c 89 ea             	mov    rdx,r13
  6782bd:	4d 89 f1             	mov    r9,r14
  6782c0:	e9 00 fe ff ff       	jmp    6780c5 <_RNvNtCs5Palesvl9wF_6mikros10univariate9aggregate+0x65>
  6782c5:	cc                   	int3   
  6782c6:	cc                   	int3   
  6782c7:	cc                   	int3   
  6782c8:	cc                   	int3   
  6782c9:	cc                   	int3   
  6782ca:	cc                   	int3   
  6782cb:	cc                   	int3   
  6782cc:	cc                   	int3   
  6782cd:	cc                   	int3   
  6782ce:	cc                   	int3   
  6782cf:	cc                   	int3   

