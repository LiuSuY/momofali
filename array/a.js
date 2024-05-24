
let a = [1,2,3,4,5];

a.map((item,index)=> {
    console.log(item,'--',index);
    if(index == 2 || item == 2) {
        return 222;
    }
})


console.log("---------");

a.forEach((item,index)=> {
    console.log(item,'--',index);
    if(index == 2 || item == 2) {
        return 111;
    }
})

for(let i in a) {
    console.log(i);
    if(i == 2) {
        break;
    }
}
while(a.length) {
    console.log(a);
    if(a.length == 4) {
        break;
    }
    a.pop();
}