import {Stack, Title} from "@mantine/core";
import {showNotification} from "@mantine/notifications";
import {IconLock, IconUser} from "@tabler/icons-react";
import {useMutation} from "@tanstack/react-query";
import {createFileRoute, redirect, useRouter} from "@tanstack/react-router";
import {z} from "zod";

import type {LoginParams} from "../../apis/auth";

import {login} from "../../apis/auth";
import {Form, useAppForm} from "../../components";
import {useAppStore} from "../../stores";
import classes from "./login.module.css";

export const Route = createFileRoute("/_common/login")({
    component: LoginPage,
    validateSearch: z.object({
        redirect: z.string()
            .optional()
            .default("/")
            .catch("/"),
    }),
    beforeLoad({search}) {
        if (useAppStore.getState().isAuthenticated) {
            const {redirect: redirectTo} = search;

            throw redirect({
                to: redirectTo,
                replace: true,
            });
        }
    },
});

function LoginPage() {
    const router = useRouter();
    const navigate = Route.useNavigate();
    const {redirect} = Route.useSearch();
    const loginMutation = useMutation({
        mutationFn: login,
    });

    const {
        AppForm,
        AppField,
        SubmitButton,
        handleSubmit,
    } = useAppForm({
        defaultValues: {
            account: "",
            password: "",
        } as LoginParams,
        validators: {
            onSubmit: z.object({
                account: z.string().nonempty("请输入账号"),
                password: z.string().nonempty("请输入密码"),
            }),
        },
        async onSubmit({value}) {
            const {accessToken} = await loginMutation.mutateAsync(value);
            useAppStore.setState(state => {
                return {
                    ...state,
                    isAuthenticated: true,
                    credentials: {
                        accessToken,
                    },
                };
            });

            await router.invalidate();
            await navigate({
                to: redirect,
                replace: true,
            });

            showNotification({
                title: "登录成功",
                message: "👏 欢迎使用Axum Rust构建的系统",
            });
        },
    });

    return (
        <div className={classes.login}>
            <Stack className={classes.form} gap="xl">
                <Title c="blue" my="xl" order={2} ta="center">基于 Rust+Axum 的起步系统</Title>

                <Form onSubmit={handleSubmit}>
                    <Stack gap="lg">
                        <AppField name="account">
                            {
                                ({TextField}) => (
                                    <TextField
                                        leftSection={<IconUser/>}
                                        placeholder="账号"
                                        radius="md"
                                        size="lg"
                                    />
                                )
                            }
                        </AppField>

                        <AppField name="password">
                            {
                                ({PasswordField}) => (
                                    <PasswordField
                                        leftSection={<IconLock/>}
                                        placeholder="密码"
                                        radius="md"
                                        size="lg"
                                    />
                                )
                            }
                        </AppField>

                        <AppForm>
                            <SubmitButton fullWidth mt="lg" radius="md" size="lg">
                                登 录
                            </SubmitButton>
                        </AppForm>
                    </Stack>
                </Form>
            </Stack>
        </div>
    );
}
