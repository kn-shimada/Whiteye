# Whiteye
## 由来
競走馬のメジロマックイーンやメジロライアンのメジロ（目白）から  
White eye → Whitey
## 構文
### 繰り返し（while）
while (条件) {   
&emsp;実行する処理1;  
&emsp;実行する処理2;  
&emsp;実行する処理3;  
&emsp;...  
}  
### 繰り返し（for）
for (初期化 ; 条件 ; 変化式) {  
&emsp;実行する処理;  
&emsp;...  
}  
※今後変更の可能性アリ
### 繰り返し（foreach）
foreach (変数 : オブジェクト) {  
&emsp;実行する処理  
}  
※範囲forの代用
### 条件分岐 (if、elsif、else)
if (条件) {  
&emsp;実行する処理;  
&emsp;...  
}  

elsif (条件) {  
&emsp;実行する処理;  
&emsp;...  
}

else {  
&emsp;実行する処理;  
&emsp;...  
}  
### 処理の強制終了（break）
while (条件) {  
&emsp;実行する処理1;  
&emsp;実行する処理2;  
&emsp;if (条件) {  
&emsp;&emsp;break  
&emsp;}  
&emsp;実行する処理3;  
}  
