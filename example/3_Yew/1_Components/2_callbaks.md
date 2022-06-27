1.Callbacks
-----------
- callback은 service,agents,부모 component와 통신하기 위해 사용됩니다.
- 내부적 볼때 callback의 타입은 clone하기 위한 Rc로 wrapped된 Fn(function)일 뿐입니다.
- callbacks는 <In>타입을 argument로 가지고 있고, 목적지에 의해  예상되는 메세지를 변환하는 emit function을 가지고 있습니다. (emit()는 보통 등록된 이벤트들을 실행시키는 함수를 의미)
- 만약 부모 component의 callback이 자식의 props로 제공된다면,자식 component는 update lifecycle hook의 콜백에서 emit을 호출하여 부모에게 메세지를 다시 보낼 수 있습니다.
- html! 매크로내에 props로 제공되는 클로져나 함수는 자동적으로 callback으로 변환됩니다.

```rust
    use yew::{html, Component, Context, Html};

    enum Msg {
        Clicked,
    }

    struct Comp;

    impl Component for Comp {

        type Message = Msg;
        type Properties = ();

        fn create(_ctx: &Context<Self>) -> Self {
            Self
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let onclick = ctx.link().callback(|_| Msg::Clicked);
            html! {
                <button {onclick}>{ "Click" }</button>
            }
        }
    }
```
