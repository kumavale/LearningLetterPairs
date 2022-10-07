// レターペアの循環リスト
// TODO: リストにする
var LP_LISTS;
var LP_POS = 0;  // 現在LP_LISTSの何番目を表示しているか

window.onload = function() {
    // シャッフルされたレターペアのリストを取得
    fetch('/shuffle_lp', { method: 'POST' })
        .then((response) => {
            return response.json();
        })
        .then((jsonobj) => {
            // グローバル変数に格納
            LP_LISTS = jsonobj.lists;
            // 問題を表示
            display_question();
        })
        .catch((e) => {
            alert("Failed to get contents");
        });

    // [Answer]ボタン押下時のイベントハンドラ設定
    var control_answer = document.querySelector('#control_answer');
    control_answer.addEventListener('click', () => { show_answer('visible'); });

    // [Next]ボタン押下時のイベントハンドラ設定
    var control_next = document.querySelector('#control_next');
    control_next.addEventListener('click', next_lp);

    // [Prev]ボタン押下時のイベントハンドラ設定
    var control_prev = document.querySelector('#control_prev');
    control_prev.addEventListener('click', prev_lp);
};

// ショートカットキーの設定
document.addEventListener('keydown', (e) => {
    switch (e.key) {
        case 'ArrowLeft':
            prev_lp();
            break;
        case 'ArrowRight':
            next_lp();
            break;
        case ' ':
            e.preventDefault();
            show_answer('visible');
            break;
        default:
            break;  // Do nothing
    }
});

// 問題を表示
function display_question() {
    var lp_name_element    = document.querySelector('#lp_name');
    var lp_objects_element = document.querySelector('#lp_objects');
    var lp_image_element   = document.querySelector('#lp_image');

    // 解答を非表示に設定
    show_answer('hidden');

    // レターペアのタイトルを設定
    lp_name_element.innerText = LP_LISTS[LP_POS].name;

    // レターペアの内容を設定
    lp_objects_element.innerHTML = '';
    LP_LISTS[LP_POS].objects.forEach(object => {
        var li = document.createElement('li');
        li.innerText = object;
        lp_objects_element.appendChild(li);
    });

    // 画像ファイルの設定
    lp_image_element.innerHTML = '';
    var img = document.createElement('img');
    img.src = "static/img/" + LP_LISTS[LP_POS].image;
    lp_image_element.appendChild(img);
}

// visibility:hiddenになっている要素を可視化する
function show_answer(visibility) {
    var lp_objects_element = document.querySelector('#lp_objects');
    var lp_image_element   = document.querySelector('#lp_image');
    lp_objects_element.style.visibility = visibility;
    lp_image_element  .style.visibility = visibility;
}

// 次の問題を表示
function next_lp() {
    if (LP_POS < LP_LISTS.length - 1) {
        LP_POS += 1;
    } else {
        LP_POS = 0;
    }
    display_question();
}

// 前の問題を表示
function prev_lp() {
    if (LP_POS > 0) {
        LP_POS -= 1;
    } else {
        LP_POS = LP_LISTS.length - 1;
    }
    display_question();
}
