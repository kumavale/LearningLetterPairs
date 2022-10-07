window.onload = function() {
    // シャッフルされたレターペアのリストを取得
    fetch('/shuffle_lp', { method: 'POST' })
        .then((response) => {
            return response.json();
        })
        .then((jsonobj) => {
            var lp_name_element    = document.querySelector('#lp_name');
            var lp_objects_element = document.querySelector('#lp_objects');
            var lp_image_element   = document.querySelector('#lp_image');

            const lists = jsonobj.lists;

            // レターペアのタイトルを設定
            lp_name_element.innerText = lists[0].name;

            // レターペアの内容を設定
            lists[0].objects.forEach(object => {
                var li = document.createElement('li');
                li.innerText = object;
                lp_objects_element.appendChild(li);
            });

            // 画像ファイルの設定
            var img = document.createElement('img');
            img.src = "static/img/" + lists[0].image;
            lp_image_element.appendChild(img);
        })
        .catch((e) => {
            alert("Failed to get contents");
        });

    // [Answer]ボタン押下時のイベントハンドラ設定
    var control_answer = document.querySelector('#control_answer');
    control_answer.addEventListener('click', show_answer);
};

// visibility:hiddenになっている要素を可視化する
function show_answer() {
    var lp_objects_element = document.querySelector('#lp_objects');
    var lp_image_element   = document.querySelector('#lp_image');
    lp_objects_element.style.visibility = 'visible';
    lp_image_element  .style.visibility = 'visible';
}
