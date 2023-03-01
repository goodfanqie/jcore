## 2023全国大学生计算机系统能力大赛操作系统设计赛-内核实现赛



## 主要简介

比赛分为两个赛道：内核赛道和功能赛道。

大赛官网地址：https://os.educg.net/

+ 内核赛道：制作一个完整的内核，通过赛方提供的用户测试，有余力的队伍还可以去支持python，rust，redis......
+ 功能与性能的比拼。

时间点：

![image-20230301202516858](https://goofanqie-1310329970.cos.ap-nanjing.myqcloud.com/blogimg/image-20230301202516858.png)

比赛奖项设置：

![image-20230301202549963](https://goofanqie-1310329970.cos.ap-nanjing.myqcloud.com/blogimg/image-20230301202549963.png)



## 目前的简要计划

组队：wj zdl

时间：3月1号开始，初赛大概是在六月截至，决赛是在八月底左右结束（如果能进的话）。

估计要学的东西：汇编+rust+操作系统+risc系统架构知识+linux下编程+qemu调试技能。

比赛文档记录和总结：github仓库放学习笔记以及代码储存啥的。

基本的编程环境：Linux（推荐虚拟机）+  vscode + vim + qemu。



### 环境搭建

https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/5setup-devel-env.html#risc-v





## 学习即参考资料

[rust-core前置教程](https://github.com/rcore-os/rCore/wiki/os-tutorial-summer-of-code-2020#step-0-%E8%87%AA%E5%AD%A6rust%E7%BC%96%E7%A8%8B%E5%A4%A7%E7%BA%A67%E5%A4%A9)

#### rcore：

+ [rcore—构建基于risc-v的类unix内核](https://rcore-os.cn/rCore-Tutorial-Book-v3/)

#### rust:

+ [rust example练习](https://doc.rust-lang.org/rust-by-example/)
+ [rust官方book](https://kaisery.github.io/trpl-zh-cn/)
+ 重点阅读《Rust 编程之道》 （第三、四、五章，理解类型系统和所有权；第九章，理解错误处理； 第十三章，理解Unsafe Rust） 
+ 完成《Rust 编程之道》第十章的完整示例代码，掌握Cargo和模块系统

#### risc-v:

- 自学[PPT for RISC-V特权指令级架构](https://content.riscv.org/wp-content/uploads/2018/05/riscv-privileged-BCN.v7-2.pdf)
- 自学[RISC-V手册：一本开源指令集的指南](http://crva.io/documents/RISC-V-Reader-Chinese-v2p1.pdf) 重点是第10章
- 自学[RIS-V特权指令级规范](https://riscv.org/specifications/privileged-isa/) 重点是与OS相关的特权硬件访问的内容
- [计算机组成与设计：RISC-V 教材](https://item.jd.com/12887758.html) 这是完整的课程教材，不要求全部看完，请根据自己的需求选择。
- [计算机组成与设计：RISC-V 浙大在线课程](http://www.icourse163.org/course/ZJU-1452997167) 这是完整的一门课，不要求全部看完，请根据自己的需求选择。
- [开发一个RISC-V上的操作系统在线课程](https://www.bilibili.com/video/BV1Q5411w7z5?from=search&seid=13099150746000866207)
- [操作系统(RISC-V)清华在线课程,2020春季](https://next.xuetangx.com/course/thu08091002729/3175284?fromArray=search_result) 这是完整的一门课，不要求全部看完，请根据自己的需求选择。
- [Rust语言操作系统的设计与实现,王润基本科毕设论文,2019](https://github.com/rcore-os/zCore/wiki/files/wrj-thesis.pdf)













































