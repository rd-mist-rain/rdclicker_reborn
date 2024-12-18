·建议:不要使用0.6.0版本之前的连点器,因为我们的核心自加入了Alt修改ticks值时就有一个严重的问题——核心部分没有加入sleep，以至于CPU的占用率过高
(这居然和原程序犯的错误一样，是令我想不到的)<br/>
·程序的GUI测试版本已经基本完成,所以我计划不再对控制台的这个版本进行任何内容更新
# rdclicker_reborn
一个简单易用的连点器程序
## 为什么会有这个程序？
这个程序如其名，是一个C++项目rdclicker(以下用"原程序“代指)的重构版(使用了Rust语言)。<br/>
至少于我现阶段来说，我所写的项目大多数是学习阶段的练手。2024年8月，随着对C++的学习暂告一段落，我不得不放弃原程序而开启一门新语言的学习之旅，而这个语言最终被我选定为Rust。<br/>
由于确实没有更多新奇的点子，以及原程序确实留下了很多遗憾(有很多我想要的功能没有实现,有很多内容我还想优化)，所以我选择用Rust重构了原程序，也就有了rdclicker_reborn(连点器程序:重生)<br/>
## 一些已知问题
* 程序使用了ANSI转义序列，但是某些控制台无法正常显示颜色而是直接显示序列本身(甚至有的序列还会被显示为乱码)。
为了解决这个问题，你可以修改Windows命令提示符的配置以使其正常显示颜色(win10及以上才有这个功能),但我们也给出了一个解决方案,只需要将程序配置文件(configs.toml)中allow_ansi一项改为false即可

## 我怎么使用这个程序？
双击运行.exe后，会有一个黑色的框框(控制台)出现，这时，你便可以使用一些功能。如果你想关闭程序，关闭控制台即可。
* 按左Ctrl，程序模拟一次在鼠标当前位置的左键点击。(连续按则持续点)程序检测按键是否按下是有间隔的，这个后面会提到。
* 按右Ctrl，程序模拟一次在鼠标当前位置的右键点击。(连续按则持续点)程序检测按键是否按下是有间隔的，这个后面会提到。
* 什么是ticks呢? ————ticks即我们之前提到的间隔，一般来说，ticks越小，间隔越短，表现为你点击的越快。ticks以毫秒为单位。<br/>
比如，你将ticks设置为100，理想化的情况下，程序每间隔100毫秒就会检测一次左、右Ctrl是否被按下，若按下就模拟一次点击。<br/>
但是，因为一些损耗，我们并不能做到ticks为10就能一秒100次点击(实测是一秒80~90次)
* 你有可能会想，怎么这么复杂！就没有一个简洁的方法吗？<br/>
————以后实现了大部分的功能，确认这个程序于Rust可行，我会为程序制作出GUI，增强程序的可控性。<br/>
### 使用配置文件更改程序的运作方式
#### 项allow_ansi
* 此项前文已经介绍过，不再做赘述
#### 项left_mode和right_mode
* 注:left_mode的所有内容都适用于right_mode项的,只是right_mode更改的是右Ctrl及其控制的鼠标右键
* left_mode接受一个整型(具体说是u8类型),现在程序只对值0和1进行处理,若你向程序传了一个非0和1的整型值,程序不会做出反应,但你也无法正常使用模拟点击的功能
* 对于left_mode,我们在程序中定义了两个模式,分别为0:长按模式,即前文介绍过的默认模式 1:单击切换模式
我们需要介绍一下单击切换模式,当你按下左/右Ctrl时(单击,200ms以内松手时有效),程序会切换当前的"点击状态",
比如,若你使用了单击切换模式,程序未处于正在模拟点击状态时,单击左Ctrl,程序会切换到模拟点击状态,不需要点击时再次单击即可停止模拟点击,
更通俗地说,单击左Ctrl时程序开始点击,再次单击左Ctrl程序结束点击
这项功能可以轻松地让你在需要执行的点击任务持续时间比较长时解放你的双手,无需长按Ctrl键
### 使用/ 指令系统控制程序的运作方式
* 要在程序中使用指令,你需要在控制台先按下/,通常这会在程序控制台中出现一个/,然后你便可以输入指令
* 指令系统实现地非常简单,几乎全部使用match匹配,所以如果你遇到了某些bug请及时向我们反馈
#### 根指令
* help 显示所有根指令(除help以外)
* config 用于执行程序配置相关内容
#### config 子指令
* help 显示所有config的子指令(除help以外)
* ticks [数字]  设置ticks的值为数字
* show 显示程序的所有配置
